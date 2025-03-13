### Utils.rs

This module contains utility functions which are used in Setup.rs module

---

## Functions

```rust
pub fn safely_from_base58_string(s: &str) -> Result<Keypair, Box<dyn std::error::Error>>
```
Decodes a base58-encoded string into a Solana `Keypair`.

### Arguments

* `s` - A base58-encoded string representing a private key.

### Returns

* `Ok(Keypair)` - A Solana `Keypair` derived from the provided string.
* `Err(Box<dyn std::error::Error>)` - An error if decoding or keypair creation fails.

### Errors

This function can fail if:
- The input string is not a valid base58-encoded private key.
- The decoded bytes do not form a valid Solana `Keypair`.

---

```rust
convert_nodes(nodes: Vec<Vec<u8>>) -> [[u8; 32]; 16384]
```

Converts a vector of byte arrays (`Vec<Vec<u8>>`) into a fixed-size array of `[u8; 32]`.

### Arguments

* `nodes` - A vector of byte arrays, each expected to be of size 32.

### Returns

* `[[u8; 32]; 16384]` - A fixed-size array of 16384 elements, each being a 32-byte array.

### Panics

This function will panic if:
- Any inner vector is not exactly 32 bytes long.
- The input vector does not contain exactly 16384 elements.

---

```rust
pub fn base58_to_array(b58_str: &str) -> Result<[u8; 32], anyhow::Error>
```

Decodes a base58-encoded string into a fixed-size `[u8; 32]` byte array.

### Arguments

* `b58_str` - A base58-encoded string.

### Returns

* `Ok([u8; 32])` - A 32-byte array decoded from the base58 string.
* `Err(anyhow::Error)` - An error if decoding fails or the decoded bytes are not exactly 32 bytes long.

### Errors

This function can fail if:
- The input string is not a valid base58-encoded value.
- The decoded bytes are not exactly 32 bytes in length.




---

> [!TIP]
> ## 📖 Generating Documentation

Apart from this readme this project also includes Rust-styled documentation for all functions and structs 
You can generate and view it locally by running on root folder:

```sh
cargo doc --no-deps --open
