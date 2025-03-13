//! # CnftNif - Rust NIFs for Solana Compressed NFTs
//!
//! This module provides Elixir bindings for interacting with Solana compressed NFTs (cNFTs).
//!
//! ## Overview
//! This library allows you to:
//! - Initialize a Merkle tree for storing compressed NFTs.
//! - Mint new compressed NFTs.
//! - Transfer ownership of a compressed NFT.
//!
//! ## Installation
//! Add `CnftNif` as a dependency in your `mix.exs`:
//!
//! ```elixir
//! def deps do
//!   [
//!     {:rustler, "~> 0.29"},
//!     {:cnft_nif, path: "native/cnft_nif"}
//!   ]
//! end
//! ```
//!
//! ## Usage Examples (IEx)
//!
//! Open an Elixir shell (`iex -S mix`) and run the following commands:
//!
//! **1. Initialize the TreeManager**
//! ```elixir
//! iex> {:ok, tree_manager} = CnftNif.tree_manager_init()
//! iex> tree_manager
//! %TreeManager{...}
//! ```
//!
//! **2. Create a Merkle Tree**
//! ```elixir
//! iex> {:ok, tree_manager} = CnftNif.tree_manager_init()
//! iex> {:ok, tree_manager, tx_hash} = CnftNif.create_merkle_tree(tree_manager, "owner_private_key")
//! iex> IO.puts("Tree created with transaction: #{tx_hash}")
//! ```
//!
//! **3. Mint a Compressed NFT**
//! ```elixir
//! iex> {:ok, tree_manager} = CnftNif.tree_manager_init()
//! iex> {:ok, tree_manager, _} = CnftNif.create_merkle_tree(tree_manager, "owner_private_key")
//! iex> {:ok, tree_manager, mint_hash} = CnftNif.mint_cnft(tree_manager, "owner_private_key", "nft_owner_pub_key")
//! iex> IO.puts("NFT minted with transaction: #{mint_hash}")
//! ```
//!
//! **4. Transfer a Compressed NFT**
//! ```elixir
//! iex> {:ok, tree_manager} = CnftNif.tree_manager_init()
//! iex> {:ok, tree_manager, _} = CnftNif.create_merkle_tree(tree_manager, "owner_private_key")
//! iex> {:ok, tree_manager, _} = CnftNif.mint_cnft(tree_manager, "owner_private_key", "nft_owner_pub_key")
//! iex> {:ok, tree_manager, transfer_hash} = CnftNif.transfer_cnft(
//! ...>   tree_manager, "tree_owner_key", "old_owner_key", "new_owner_pub_key", 1, "data_hash", "creator_hash"
//! ...> )
//! iex> IO.puts("NFT transferred with transaction: #{transfer_hash}")
//! ```


pub mod setup;
pub mod utils;

use setup::TreeManager;



#[rustler::nif]
pub fn tree_manager_init () -> TreeManager{
    TreeManager::default()
}




#[rustler::nif]
pub fn create_merkle_tree(tree_manager: TreeManager, owner_private_key: &str) -> Result<(TreeManager, String), String>{
    let mutable_tree_manager = &mut tree_manager.clone();
    let txn_hash = mutable_tree_manager.create_tree(owner_private_key);

    match txn_hash {
        Ok(hash) => Ok((mutable_tree_manager.clone(), hash.to_string())),
        Err(e) => Err(format!("Error: \n {e}"))
    }

}



#[rustler::nif]
pub fn mint_cnft(tree_manager: TreeManager, owner_private_key: &str, nft_owner_pub_key: &str) -> Result<(TreeManager, String), String>{
    let mutable_tree_manager = &mut tree_manager.clone();
    let txn_hash = mutable_tree_manager.mint_cnft(owner_private_key, nft_owner_pub_key);

    match txn_hash {
        Ok(hash) => Ok((mutable_tree_manager.clone(), hash.to_string())),
        Err(e) => Err(format!("Error: \n {e}"))
    }

}



#[rustler::nif]
pub fn transfer_cnft(tree_manager: TreeManager,
    owner_private_key: &str, 
    old_owner_private_key: &str, 
    new_owner_pub_key: &str,
    index: usize,
    data_hash: &str,
    creator_hash: &str
    ) -> Result<(TreeManager, String), String>{
    let mutable_tree_manager = &mut tree_manager.clone();
    let txn_hash = mutable_tree_manager.transfer_cnft(
        owner_private_key,
        old_owner_private_key, 
        new_owner_pub_key,
        index,
        data_hash,
        creator_hash
    );

    match txn_hash {
        Ok(hash) => Ok((mutable_tree_manager.clone(), hash.to_string())),
        Err(e) => Err(format!("Error: \n {e}"))
    }

}


rustler::init!("Elixir.CnftNif");
