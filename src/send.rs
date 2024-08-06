use crate::utils;
use dialoguer::Input;
use ethers::prelude::*;
use std::sync::Arc;
use ethers::types::transaction::eip2718::TypedTransaction;

pub async fn send_eth() {
    let provider = utils::get_provider();
    let wallet = utils::get_wallet();
    let client = SignerMiddleware::new(provider.clone(), wallet);

    // Prompt user for recipient address
    let recipient_address: String = Input::new()
        .with_prompt("Enter the recipient's wallet address")
        .interact_text()
        .unwrap();

    let to: Address = match recipient_address.parse() {
        Ok(addr) => addr,
        Err(_) => {
            println!("❌ Invalid recipient address.");
            return;
        }
    };

    // Prompt user for the amount of ETH to send
    let amount_eth: f64 = Input::new()
        .with_prompt("Enter the amount of ETH to send")
        .interact_text()
        .unwrap();

    let value_in_wei = ethers::utils::parse_units(amount_eth, 18).unwrap();

    let tx: TypedTransaction = TransactionRequest::new().to(to).value(value_in_wei).into();

    // Estimate gas fee
    let gas_fee_in_wei = match utils::estimate_gas_fee(&client, &tx).await {
        Ok(fee) => fee,
        Err(err) => {
            println!("❌ Failed to estimate gas fee: {}", err);
            return;
        }
    };

    let gas_fee_in_eth = ethers::utils::format_units(gas_fee_in_wei, 18).unwrap();
    println!("💡 Estimated Gas Fee: ~{} ETH", gas_fee_in_eth);

    // Confirm the transaction
    let confirmation: String = Input::new()
        .with_prompt(format!(
            "The total transaction (including gas fee) is ~{} ETH. Do you want to proceed? (yes/no)",
            gas_fee_in_eth
        ))
        .default("no".to_string())
        .interact_text()
        .unwrap();

    if confirmation.to_lowercase() != "yes" {
        println!("❌ Transaction canceled by user.");
        return;
    }

    // Send the transaction
    println!("📤 Sending transaction...");
    match Arc::new(client).send_transaction(tx.clone(), None).await {
        Ok(pending_tx) => {
            println!("✅ Transaction sent successfully!");
            println!("Tx Hash: {:?}", pending_tx.tx_hash());
        }
        Err(err) => {
            println!("❌ Error sending transaction: {}", err);
        }
    }
}
