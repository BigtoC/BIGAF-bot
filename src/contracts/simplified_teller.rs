use alloy::sol;
use alloy_primitives::Address;

sol! {
    #[sol(rpc)]
    contract SimplifiedTeller {
        function deposit(address depositAsset, uint256 depositAmount, uint256 minimumMint);
        function withdraw(address withdrawAsset, uint256 shareAmount, uint256 minimumAssets);
    }
}

pub struct SimplifiedTellerContract {
    address: Address,
    rpc_url: String,
}