mod setup;
mod utils;

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
