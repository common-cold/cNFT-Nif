defmodule CnftNif do
    use Rustler, otp_app: :cnft_project, crate: "cnftnif"
    def tree_manager_init(), do: :erlang.nif_error(:nif_not_loaded);
    def create_merkle_tree(_tree_manager, _owner_private_key), do: :erlang.nif_error(:nif_not_loaded);
    def mint_cnft(_tree_manager, _owner_private_key, _nft_owner_pub_key), do: :erlang.nif_error(:nif_not_loaded);
    def transfer_cnft(_tree_manager, _owner_private_key, _old_owner_private_key, _new_owner_pub_key, _index, _data_hash, _creator_hash), do: :erlang.nif_error(:nif_not_loaded);
  end
  