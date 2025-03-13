# CnftNif - Rust NIFs for Solana Compressed NFTs

This module provides Elixir bindings for interacting with Solana compressed NFTs (cNFTs).

## Overview
This library allows you to:
- Initialize a Merkle tree for storing compressed NFTs.
- Mint new compressed NFTs.
- Transfer ownership of a compressed NFT.

## Installation
Add `CnftNif` as a dependency in your `mix.exs`:

```elixir
def deps do
[
{:rustler, "~> 0.29"},
{:cnft_nif, path: "native/cnft_nif"}
]
end
```

## Usage Examples (IEx)

Open an Elixir shell (`iex -S mix`) and run the following commands:

**1. Initialize the TreeManager**
```elixir
iex> {:ok, tree_manager} = CnftNif.tree_manager_init()
iex> tree_manager
%TreeManager{...}
```

**2. Create a Merkle Tree**
```elixir
iex> {:ok, tree_manager} = CnftNif.tree_manager_init()
iex> {:ok, tree_manager, tx_hash} = CnftNif.create_merkle_tree(tree_manager, "owner_private_key")
iex> IO.puts("Tree created with transaction: #{tx_hash}")
```

**3. Mint a Compressed NFT**
```elixir
iex> {:ok, tree_manager} = CnftNif.tree_manager_init()
iex> {:ok, tree_manager, _} = CnftNif.create_merkle_tree(tree_manager, "owner_private_key")
iex> {:ok, tree_manager, mint_hash} = CnftNif.mint_cnft(tree_manager, "owner_private_key", "nft_owner_pub_key")
iex> IO.puts("NFT minted with transaction: #{mint_hash}")
```

**4. Transfer a Compressed NFT**
```elixir
iex> {:ok, tree_manager} = CnftNif.tree_manager_init()
iex> {:ok, tree_manager, _} = CnftNif.create_merkle_tree(tree_manager, "owner_private_key")
iex> {:ok, tree_manager, _} = CnftNif.mint_cnft(tree_manager, "owner_private_key", "nft_owner_pub_key")
iex> {:ok, tree_manager, transfer_hash} = CnftNif.transfer_cnft(
...>   tree_manager, "tree_owner_key", "old_owner_key", "new_owner_pub_key", 1, "data_hash", "creator_hash"
...> )
iex> IO.puts("NFT transferred with transaction: #{transfer_hash}")
```
