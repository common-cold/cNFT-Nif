### Setup.rs

This module implements the core logic for our cNFT NIFs

---

## Struct

```rust
#[derive(NifStruct, Clone)]
#[module = "CnftNif.TreeManager"]
pub struct TreeManager {
    pub max_depth: usize,
    pub max_buffer_size: usize,
    pub serialized_tree_account: Vec<u8>,
    pub nodes: Vec<Vec<u8>>,
    pub minted: usize,
}
```

Represents the `TreeManager`, responsible for managing an off-chain Merkle tree 
used in compressed NFTs (cNFTs).

This struct helps maintain the state and structure of the Merkle tree while interacting 
with Elixir through Rust NIFs. Due to serialization issues, certain tree components 
are stored as raw data instead of structured objects.

Fields:
- `max_depth`: Maximum depth of the Merkle tree.
- `max_buffer_size`: Maximum buffer size of the Merkle tree
- `serialized_tree_account`: Serialized representation of the Merkle tree account keypair.
- `nodes`: Serialized representation of Tree nodes of the merkle tree.
  - Instead of storing a `MerkleTree` object directly (which caused serialization/deserialization issues with Elixir),
    this struct keeps an array of hashed leaf nodes (`LeafSchema` objects).
  - These nodes allow for constructing a local off-chain Merkle tree, enabling root calculation 
    and proof generation.
- `minted`: Number of minted cNFTs, also used to generate asset IDs and nonce values 
  within the `LeafSchema`.

---

## Struct Implemented Functions

```rust
pub fn get_minted(&self) -> usize
```  
Returns the number of minted cNFTs.  

---

```rust
pub fn get_proof(&self, merkle_tree: &MerkleTree, index: usize) -> Vec<Node>
```
Get proof of the off-chain merkle tree

---

```rust
pub fn create_tree(&mut self, owner_private_key: &str) -> Result<String, String>
```
Creates a new Merkle tree on Solana by initializing the tree account and configuration.

This function uses the owner's private key (in base58 format) to derive the tree owner,
calculates the required size for the tree account (including the concurrent Merkle tree header),
fetches the minimum balance for rent exemption, and constructs the necessary system and configuration
instructions. It then signs and sends the transaction, returning the transaction signature as a string.


> [!NOTE]
> For now it only supports tress with MAX_DEPTH = 14 and MAX_BUFFER_SIZE = 64


### Parameters

* `owner_private_key` - A string slice representing the owner's private key in base58 format.
                       This key is used both as the payer for account creation and as the tree creator.

### Returns

* `Ok(String)` - On success, returns the transaction signature as a string.
* `Err(String)`  - Returns an error message if any step (decoding, rent calculation, transaction sending, etc.) fails.

---

```rust
pub fn mint_cnft(&mut self, owner_private_key: &str, nft_owner: &str) -> Result<String, String> 
```
Mints a new Compressed NFT (cNFT) to the specified owner within the Merkle tree.
This function constructs a new metadata entry, signs a mint transaction, and submits it to the Solana blockchain.
The function also updates the nodes of the current TreeManager instance after successfull minting.
     
### Parameters

* `owner_private_key` - A string slice representing the private key of the tree owner in base58 format.
                       This key is used to sign the transaction.
* `nft_owner` - A string slice representing the public key of the recipient in base58 format.

### Returns

* `Ok(String)` - On success, returns the transaction signature of the mint operation.
* `Err(anyhow::Error)` - Returns an error if any step fails (invalid keys, transaction failure, etc.).

---

```rust
 pub fn transfer_cnft(
        &mut self,
        tree_owner_private_key: &str, 
        old_owner_private_key: &str, 
        new_owner_pub_key: &str,
        index: usize,
        data_hash: &str,
        creator_hash: &str
    ) -> Result<String, anyhow::Error>
```
Transfers a compressed NFT (cNFT) from one owner to another within the Merkle tree.

This function updates the off-chain Merkle tree, constructs a valid proof,  
and submits a Solana transaction to transfer ownership of the cNFT.

### Arguments

* `tree_owner_private_key` - The private key of the tree owner, used to authorize the transfer.
* `old_owner_private_key` - The private key of the current NFT owner, required for signing the transfer.
* `new_owner_pub_key` - The public key of the new NFT owner who will receive the transferred NFT.
* `index` - The index of the NFT within the Merkle tree.
* `data_hash` - The base58-encoded hash of the NFT metadata.
* `creator_hash` - The base58-encoded hash of the NFT creators.

### Returns

* `Ok(String)` - A transaction signature confirming the successful transfer.
* `Err(anyhow::Error)` - An error if the transfer fails.
