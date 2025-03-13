use solana_sdk::{bs58, signature::Keypair};


pub fn safely_from_base58_string(s: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    let bytes = bs58::decode(s).into_vec()?;
    let keypair = Keypair::from_bytes(&bytes)?;
    Ok(keypair)
}

pub fn convert_nodes(nodes: Vec<Vec<u8>>) -> [[u8; 32]; 16384]{
    let result: Vec<[u8; 32]> = nodes.into_iter()
        .map(|inner| inner
        .try_into().expect("Error occurred while converting nodes to vec<[u8; 32]>"))
        .collect();

    result.try_into().expect("Error occurred while converting vec to [[u8; 32]]")
}

// pub fn convert_vec_hash_to_array(vec_hash: Vec<u8>) -> Result<[u8; 32], Vec<u8>> {
//     vec_hash.try_into()
// }


pub fn base58_to_array(b58_str: &str) -> Result<[u8; 32], anyhow::Error> {
    let bytes = bs58::decode(b58_str).into_vec()?;
    
    Ok(bytes.as_slice().try_into()?)
}
