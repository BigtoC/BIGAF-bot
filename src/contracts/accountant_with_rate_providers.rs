use alloy::sol;

sol! {
    #[sol(rpc)]
    contract AccountantWithRateProviders {
        function getRate() public view returns (uint256);
    }
}
