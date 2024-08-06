use crate::utils;
use ethers::prelude::*;

pub async fn latest_transactions() {
    let provider = utils::get_provider();
    let wallet = utils::get_wallet();

    match provider
        .get_logs(&Filter::new().from_block(0).address(vec![wallet.address()]))
        .await
    {
        Ok(logs) if logs.is_empty() => println!("❌ No transactions found."),
        Ok(logs) => {
            println!("✅ Latest transactions:");
            for log in logs.iter().take(10) {
                println!("Tx Hash: {:?}", log.transaction_hash);
            }
        }
        Err(err) => println!("❌ Error fetching transactions: {}", err),
    }
}
