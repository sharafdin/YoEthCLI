use ethers::prelude::*;
use crate::utils;

pub async fn view_balance() {
    let provider = utils::get_provider();
    let wallet = utils::get_wallet();

    match provider.get_balance(wallet.address(), None).await {
        Ok(balance) => {
            let balance_eth = ethers::utils::format_units(balance, 18).unwrap();
            println!(
                "✅ Your Wallet Balance: {} ETH",
                balance_eth
            );
        }
        Err(err) => {
            eprintln!("❌ Error fetching balance: {}", err);
        }
    }
}