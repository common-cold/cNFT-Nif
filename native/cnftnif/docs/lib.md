# Lib.rs

This module provides Elixir bindings for interacting with Solana compressed NFTs (cNFTs).
These functions are exposed as a Native Implemented Function (NIF) using Rustler, allowing Elixir to call it

---

## Rust NIFs


```rust
#[rustler::nif]
pub fn tree_manager_init () -> TreeManager
```
Initializes and returns a new `TreeManager` instance with default settings.

This function is exposed as a Native Implemented Function (NIF) using Rustler,
allowing Elixir to call it and obtain a default-initialized `TreeManager`.

### Returns:
- A new `TreeManager` instance initialized with default values.

---

```rust
#[rustler::nif]
pub fn create_merkle_tree(tree_manager: TreeManager, owner_private_key: &str) -> Result<(TreeManager, String), String>
```

Creates a new Merkle tree and returns the updated `TreeManager` along with the transaction hash.

### Parameters:
- `tree_manager`: The current `TreeManager` instance.
- `owner_private_key`: A string representing the owner's private key used to create the tree.

### Returns:
- `Ok((TreeManager, String))`: The updated `TreeManager` and the transaction hash if successful.
- `Err(String)`: An error message if the Merkle tree creation fails.


---

```rust
#[rustler::nif]
pub fn mint_cnft(tree_manager: TreeManager, owner_private_key: &str, nft_owner_pub_key: &str) -> Result<(TreeManager, String), String>
```

Mints a compressed NFT (cNFT) and returns the updated `TreeManager` along with the transaction hash.

### Parameters:
- `tree_manager`: The current `TreeManager` instance.
- `owner_private_key`: The private key of the tree owner, used to authorize the minting.
- `nft_owner_pub_key`: The public key of the recipient who will own the minted cNFT.

### Returns:
- `Ok((TreeManager, String))`: The updated `TreeManager` and the transaction hash if minting is successful.
- `Err(String)`: An error message if minting fails.


---

```rust
#[rustler::nif]
pub fn transfer_cnft(tree_manager: TreeManager,
    owner_private_key: &str, 
    old_owner_private_key: &str, 
    new_owner_pub_key: &str,
    index: usize,
    data_hash: &str,
    creator_hash: &str
    ) -> Result<(TreeManager, String), String>
```

Transfers a compressed NFT (cNFT) to a new owner and returns the updated `TreeManager` along with the transaction hash.

This function is exposed as a Rustler NIF, allowing Elixir to perform a cNFT transfer by 
providing the necessary ownership details and metadata.

### Parameters:
- `tree_manager`: The current `TreeManager` instance.
- `owner_private_key`: The private key of the Merkle tree owner.
- `old_owner_private_key`: The private key of the current cNFT owner.
- `new_owner_pub_key`: The public key of the new owner receiving the cNFT.
- `index`: The index of the cNFT in the Merkle tree.
- `data_hash`: A hash representing additional metadata of the cNFT.
- `creator_hash`: A hash representing the creator details of the cNFT.

### Returns:
- `Ok((TreeManager, String))`: The updated `TreeManager` and the transaction hash if the transfer is successful.
- `Err(String)`: An error message if the transfer fails.


