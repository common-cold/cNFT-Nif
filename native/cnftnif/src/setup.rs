
use std::str::FromStr;

use anyhow::{anyhow, bail};
use mpl_bubblegum::{accounts::TreeConfig, hash::{hash_creators, hash_metadata}, instructions::{CreateTreeConfigBuilder, MintV1Builder, TransferBuilder}, programs::{SPL_ACCOUNT_COMPRESSION_ID, SPL_NOOP_ID}, types::{LeafSchema, MetadataArgs, TokenProgramVersion, TokenStandard}, utils::get_asset_id};
use once_cell::sync::Lazy;
use rustler::NifStruct;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, instruction::AccountMeta, pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction, transaction::Transaction};
use spl_account_compression::{state::CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1, ConcurrentMerkleTree};
use spl_merkle_tree_reference::{MerkleTree, Node};

use crate::utils::{base58_to_array, convert_nodes, safely_from_base58_string};


static RPC_CLIENT: Lazy<RpcClient> = Lazy::new(|| {
    RpcClient::new_with_commitment(
        "https://solana-devnet.g.alchemy.com/v2/IA5XqK-rU0LYpFekBWARC-2_lWQNqmFG",
        CommitmentConfig::confirmed()
    )
});




#[derive(NifStruct, Clone)]
#[module = "CnftNif.TreeManager"]
pub struct TreeManager {
    pub max_depth: usize,
    pub max_buffer_size: usize,
    pub serialized_tree_account: Vec<u8>,
    pub nodes: Vec<Vec<u8>>,
    minted: usize
}

impl Default for TreeManager {
    fn default() -> Self {
        let keypair = Keypair::new();
        let nodes = (0..16384).map(|_| vec![0; 32]).collect();
        Self {
            max_depth: 14, 
            max_buffer_size: 64,
            serialized_tree_account: keypair.to_bytes().to_vec(),
            nodes: nodes,
            minted: 0
        }
    }
}

impl TreeManager {
    
    pub fn get_minted (&self) -> usize {
        self.minted
    }

    pub fn get_proof (&self, merkle_tree: &MerkleTree, index: usize) -> Vec<Node>{
        merkle_tree.get_proof_of_leaf(index)
    }

    pub fn create_tree(&mut self, owner_private_key: &str) -> Result<String, String> {
        const MAX_DEPTH: usize= 14;
        const MAX_BUFFER_SIZE: usize = 64;
        let tree_account = Keypair::from_bytes(self.serialized_tree_account.as_slice()).map_err(|e| e.to_string())?;

        
        if owner_private_key.trim().is_empty() {
            return Err("owner_private_key parameter must be provided".to_string());
        }
        let tree_owner = safely_from_base58_string(owner_private_key).map_err(|e| e.to_string())?;  
    
        let size = CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1 + 
            std::mem::size_of::<ConcurrentMerkleTree<MAX_DEPTH, MAX_BUFFER_SIZE>>();
        
        let rent = RPC_CLIENT.get_minimum_balance_for_rent_exemption(size).map_err(|e| e.to_string())?;
        
        
        let (tree_config, _) = TreeConfig::find_pda(&tree_account.pubkey());
    
        
        let tree_account_ix = system_instruction::create_account(
            &tree_owner.pubkey(),
            &tree_account.pubkey(),
            rent,
            size as u64,
            &spl_account_compression::ID
        );
     
        let tree_config_ix = CreateTreeConfigBuilder::new()
            .tree_config(tree_config)
            .payer(tree_owner.pubkey())
            .merkle_tree(tree_account.pubkey())
            .tree_creator(tree_owner.pubkey())
            .log_wrapper(SPL_NOOP_ID)
            .compression_program(SPL_ACCOUNT_COMPRESSION_ID)
            .system_program(solana_program::system_program::id())
            .max_depth(MAX_DEPTH as u32)
            .max_buffer_size(MAX_BUFFER_SIZE as u32)
            .public(false)
            .instruction();
    
    
        let tree_txn = Transaction::new_signed_with_payer(
            &[tree_account_ix, tree_config_ix],
            Some(&tree_owner.pubkey()),
            &[&tree_account, &tree_owner],
            RPC_CLIENT.get_latest_blockhash().map_err(|e| e.to_string())? 
        );
    
        let sig = RPC_CLIENT.send_and_confirm_transaction(&tree_txn).map_err(|e| e.to_string())?;
    
        Ok(sig.to_string())
        
    }


    pub fn mint_cnft(&mut self, owner_private_key: &str, nft_owner: &str) -> Result<String, anyhow::Error> {

        let tree_account = Keypair::from_bytes(self.serialized_tree_account.as_slice())
            .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;
        
        if owner_private_key.trim().is_empty() {
            bail!("owner_private_key parameter must be provided, {}:{}", file!(), line!());
        }
        let tree_owner = safely_from_base58_string(owner_private_key)
            .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;  

        let nft_owner = Pubkey::from_str(nft_owner)
            .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;
        
        let (tree_config, _) = TreeConfig::find_pda(&tree_account.pubkey());
    
    
        let metadata = MetadataArgs {
            name: format!("Prajjwal's cnft {}", self.minted),
            symbol: String::from("PcNFT"),
            uri: String::from("https://cdn.100xdevs.com/metadata.json"),
            seller_fee_basis_points: 0,
            primary_sale_happened: true,
            is_mutable: true,
            edition_nonce: None,
            token_standard: Some(TokenStandard::NonFungible),
            collection: None,
            uses: None,
            token_program_version: TokenProgramVersion::Original,
            creators: vec![],
        };

    
        
        let mint_ix = MintV1Builder::new()
            .leaf_delegate(nft_owner)
            .leaf_owner(nft_owner)
            .merkle_tree(tree_account.pubkey())
            .payer(tree_owner.pubkey())
            .tree_config(tree_config)
            .tree_creator_or_delegate(tree_owner.pubkey())
            .metadata(metadata.clone())
            .instruction();
    
        let mint_txn = Transaction::new_signed_with_payer(
            &[mint_ix],
            Some(&tree_owner.pubkey()),
            &[&tree_owner],
            RPC_CLIENT.get_latest_blockhash().map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))? 
        );
   
    
        let sig = RPC_CLIENT.send_and_confirm_transaction(&mint_txn)
            .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;

        let minted_nonce = self.get_minted();
        let data_hash = hash_metadata(&metadata)
            .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;
        let creator_hash = hash_creators(&metadata.creators);
        let asset_id  = get_asset_id(&tree_account.pubkey(), minted_nonce as u64);
        
        let leaf = LeafSchema::V1 { 
            id: asset_id,
            owner: nft_owner,
            delegate: nft_owner, 
            nonce: minted_nonce as u64, 
            data_hash: data_hash, 
            creator_hash: creator_hash 
        };

        self.nodes[minted_nonce] = leaf.hash().to_vec();

        self.minted += 1;

        Ok(sig.to_string())
    }



    pub fn transfer_cnft(
        &mut self,
        tree_owner_private_key: &str, 
        old_owner_private_key: &str, 
        new_owner_pub_key: &str,
        index: usize,
        data_hash: &str,
        creator_hash: &str
    ) -> Result<String, anyhow::Error> {
        
        let leaves: [Node; 16384] = convert_nodes(self.nodes.clone());
        let off_chain_merkle_tree = MerkleTree::new(&leaves);

        let proof: Vec<AccountMeta> = self.get_proof(&off_chain_merkle_tree, index)
            .iter()
            .map(|node| AccountMeta {
                pubkey: Pubkey::new_from_array(*node),
                is_signer: false,
                is_writable: false,
            })
            .collect();

        let data_hash_as_array = base58_to_array(data_hash)
            .map_err(|e| anyhow!("Error while converting data hash: {}, {}:{}", e, file!(), line!()))?;

        let creator_hash_as_array = base58_to_array(creator_hash)
            .map_err(|e| anyhow!("Error while converting creator hash: {}, {}:{}", e, file!(), line!()))?;
        
        let tree_account = Keypair::from_bytes(self.serialized_tree_account.as_slice())
            .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;
        
        if tree_owner_private_key.trim().is_empty() {
            bail!("owner_private_key parameter must be provided, {}:{}", file!(), line!());
        }
        let tree_owner = safely_from_base58_string(tree_owner_private_key)
            .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;  


        if old_owner_private_key.trim().is_empty() {
                bail!("old_owner_private_key parameter must be provided, {}:{}", file!(), line!());
        }
        let old_owner = safely_from_base58_string(old_owner_private_key)
                .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;  
        

        let new_owner = Pubkey::from_str(new_owner_pub_key)
            .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;


        let (tree_config, _) = TreeConfig::find_pda(&tree_account.pubkey());

        


        let transfer_ix = TransferBuilder::new()
            .tree_config(tree_config)
            .leaf_owner(old_owner.pubkey(), true)
            .leaf_delegate(old_owner.pubkey(), false)
            .new_leaf_owner(new_owner)
            .merkle_tree(tree_account.pubkey())
            .log_wrapper(SPL_NOOP_ID)
            .compression_program(SPL_ACCOUNT_COMPRESSION_ID)
            .system_program(solana_program::system_program::id())
            .root(off_chain_merkle_tree.root)
            .data_hash(data_hash_as_array)
            .creator_hash(creator_hash_as_array)
            .nonce(index as u64)
            .index(index as u32)
            .add_remaining_accounts(&proof)
            .instruction();
        

        let transfer_txn = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&tree_owner.pubkey()),
            &[&old_owner, &tree_owner],
            RPC_CLIENT.get_latest_blockhash().map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))? 
        );


        let sig = RPC_CLIENT.send_and_confirm_transaction(&transfer_txn)
        .map_err(|e| anyhow!("Error: {}, {}:{}", e, file!(), line!()))?;

    
        let asset_id  = get_asset_id(&tree_account.pubkey(), index as u64);
        
        let leaf = LeafSchema::V1 { 
            id: asset_id,
            owner: new_owner,
            delegate: new_owner, 
            nonce: index as u64, 
            data_hash: data_hash_as_array, 
            creator_hash: creator_hash_as_array 
        };

        self.nodes[index] = leaf.hash().to_vec();

        Ok(sig.to_string())

    }
}
