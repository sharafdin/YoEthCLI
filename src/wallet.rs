use ethers::prelude::*;
use ethers::core::k256::ecdsa::SigningKey;
use dialoguer::Input;
use hex::encode;
use rand::thread_rng;

/// Create a new wallet
pub fn create_wallet() {
    let wallet: Wallet<SigningKey> = LocalWallet::new(&mut thread_rng());
    let private_key = encode(wallet.signer().to_bytes());
    let full_address = format!("0x{}", hex::encode(wallet.address().as_bytes()));

    println!("⚠️  Important Instructions!");
    println!("1. Your private key is your ONLY way to access your funds. If you lose it, your wallet and funds will be permanently inaccessible.");
    println!("2. Store your private key in a secure place where nobody else can access it.");
    println!("3. Use your wallet address (below) to receive ETH or tokens. Do NOT share your private key with anyone.");
    println!(
        "4. If you have your private key, you can always retrieve your wallet address at any time."
    );
    println!("5. Remember: Even if you know your wallet address, you cannot retrieve your private key. The private key is what proves ownership of the funds in your wallet.");
    println!("6. Blockchain systems like Ethereum rely entirely on private keys for security and access. Losing it means losing access forever.");

    println!("\n✅ Your New Wallet Details:");
    println!("   Private Key: {}", private_key);
    println!("   Wallet Address: {}", full_address);
}

/// Retrieve wallet address from private key
pub fn retrieve_wallet_address() {
    let private_key: String = Input::new()
        .with_prompt("Enter your private key to retrieve your wallet address")
        .interact_text()
        .unwrap();

    match private_key.parse::<LocalWallet>() {
        Ok(wallet) => {
            let full_address = format!("0x{}", hex::encode(wallet.address().as_bytes()));
            println!("✅ Your Wallet Address: {}", full_address);
        }
        Err(_) => {
            println!("❌ Invalid private key. Please create a new wallet if you don't have one.");
        }
    }
}
