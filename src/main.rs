mod wallet;
mod send;
mod transactions;
mod balance;
mod utils;

use dialoguer::{theme::ColorfulTheme, Select};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let actions = vec![
        "💰 Generate a New Ethereum Wallet",
        "📤 Send Ethereum (ETH) to Someone",
        "💳 Check Your Wallet Balance",
        "🔍 Explore Recent Transactions",
        "🔑 Retrieve Wallet Address Using Private Key",
        "🚪 Exit ethereumCLI",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Welcome to ethereumCLI! What do you want to do?")
        .items(&actions)
        .default(0)
        .interact()?;

    match selection {
        0 => wallet::create_wallet(),
        1 => send::send_eth().await,
        2 => balance::view_balance().await,
        3 => transactions::latest_transactions().await,
        4 => wallet::retrieve_wallet_address(),
        5 => {
            println!("Goodbye!");
            return Ok(());
        }
        _ => println!("Invalid selection."),
    }

    Ok(())
}
