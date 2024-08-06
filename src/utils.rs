use ethers::prelude::*;
use std::env;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::core::k256::ecdsa::SigningKey;

pub fn get_provider() -> Provider<Http> {
    Provider::<Http>::try_from(format!(
        "https://mainnet.infura.io/v3/{}",
        env::var("INFURA_API_KEY").unwrap()
    ))
    .unwrap()
}

pub fn get_wallet() -> LocalWallet {
    env::var("PRIVATE_KEY")
        .unwrap()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(1u64)
}

pub async fn estimate_gas_fee(
    client: &SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    tx: &TypedTransaction,
) -> Result<U256, String> {
    match client.estimate_gas(tx, None).await {
        Ok(gas_estimate) => match client.provider().get_gas_price().await {
            Ok(gas_price) => Ok(gas_price * gas_estimate),
            Err(err) => Err(format!("Failed to fetch gas price: {}", err)),
        },
        Err(err) => Err(format!("Failed to estimate gas: {}", err)),
    }
}
