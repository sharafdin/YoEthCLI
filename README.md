# YoEthCLI

YoEthCLI is a professional and secure command-line Ethereum wallet built for developers and enthusiasts. It allows users to interact with the Ethereum blockchain for sending ETH, checking balances, generating wallets, and exploring transactions – all with transparency and security in mind.

---

## 🚀 Features

- **Generate a Wallet**: Create a new Ethereum wallet with a private key and wallet address.
- **Send Ethereum**: Transfer ETH securely to any address on the Ethereum blockchain.
- **Check Balance**: Instantly check the ETH balance of your wallet.
- **Transaction History**: View the latest transactions for your wallet.
- **Retrieve Address**: Extract your wallet address from a private key if needed.
- **Developer-Friendly**: Built for developers who want a transparent and auditable CLI Ethereum wallet.

---

## 🔍 Why YoEthCLI?

The principle of cryptocurrency is **"Don't trust, verify"**. With YoEthCLI, you get complete transparency:
- The code is open-source, allowing you to inspect every function.
- **YoEthCLI does not store or share your private key** – you are in complete control.

💡 **Tip**: Always verify the code and the tool you’re using. Never blindly trust anyone in the crypto space, including me. 

---

## ⚙️ Getting Started

### 1. Requirements

- Rust (for building the CLI wallet)
- Cargo (Rust package manager)
- Ethereum private key
- Infura API Key (or equivalent provider API)

---

## 📄 Configuration

To run **YoEthCLI**, you need to configure some environment variables. A sample `.env-example` file is provided to help you get started.

### Steps to Configure:
1. Locate the `.env-example` file in the project directory.
2. Copy it to create a new `.env` file:
   ```bash
   cp .env-example .env
   ```
3. Open the `.env` file and replace the placeholder values with your actual credentials:
   - `PRIVATE_KEY`: Your Ethereum private key (keep this secret and secure).
   - `INFURA_API_KEY`: Your Infura project API key for interacting with the Ethereum blockchain.

### Example `.env` File:
```plaintext
# Your Ethereum private key (KEEP THIS SECRET AND NEVER SHARE IT)
PRIVATE_KEY=your_private_key_here

# Infura API key for accessing Ethereum blockchain data
INFURA_API_KEY=your_infura_api_key_here
```

> ⚠️ **Note**: Never share your `.env` file or its contents with anyone. Your private key is critical for accessing your Ethereum wallet and must remain secure.

---

## 🔧 Usage

### 1. Build the CLI
To compile the YoEthCLI wallet:
```bash
cargo build --release
```

### 2. Run the CLI
To start YoEthCLI:
```bash
cargo run
```

---

## 💻 Actions

When you run YoEthCLI, you’ll see the following menu options:

1. **💰 Generate a New Crypto Wallet**  
   Create a new wallet and securely store your private key and wallet address.

2. **📤 Send Ethereum (ETH) to Someone**  
   Enter the recipient’s address and amount to securely transfer ETH.

3. **💳 Check Your Wallet Balance**  
   Instantly fetch your current ETH balance.

4. **🔍 Explore Recent Blockchain Transactions**  
   View up to 10 recent transactions for your wallet.

5. **🔑 Retrieve Wallet Address Using Private Key**  
   Get your wallet address by providing a valid private key.

6. **🚪 Exit YoEthCLI**  
   Safely exit the program.

---

## 📚 Useful Resources

- [Ethereum Official Documentation](https://ethereum.org/en/developers/)
- [Infura](https://infura.io/) – Ethereum API provider
- [Etherscan](https://etherscan.io/) – Ethereum block explorer

---

## 💡 Tips for Security

- Never share your private key or `.env` file with anyone.
- Always verify the code before running it.
- Use a secure and encrypted location to store your `.env` file.

---

## 🛠️ Contributing

Feel free to fork this repository and contribute improvements. Submit pull requests, and they will be reviewed promptly.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
