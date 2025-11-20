use alloy::sol;

sol! {
    #[sol(rpc)]
    contract SimplifiedTeller {
        function deposit(address depositAsset, uint256 depositAmount, uint256 minimumMint);
        function withdraw(address withdrawAsset, uint256 shareAmount, uint256 minimumAssets);
    }
}
