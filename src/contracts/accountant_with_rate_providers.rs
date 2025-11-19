use alloy::sol;
use alloy_primitives::Address;

sol! {
    #[sol(rpc)]
    contract AccountantWithRateProviders {
        function getRateSafe() external view returns (uint256 rate);
    }
}

pub struct AccountantWithRateProvidersContract {
    address: Address,
    rpc_url: String,
}
