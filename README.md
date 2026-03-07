# Ghost Army Manager (GAM-rs) 🦀👻⚔️

[![CI](https://github.com/padma02-drmn/ghost-army-manager/actions/workflows/ci.yml/badge.svg)](https://github.com/padma02-drmn/ghost-army-manager/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A high-performance CLI tool for managing multiple "Shadow Wallets" for airdrop farming, testnet automation, and large-scale blockchain interaction.

## 🚀 Key Features
- **Deterministic Generation**: Mass-generate unique EVM-compatible wallets.
- **Secure by Design**: Automatic encryption and local-first storage (no data ever leaves your machine).
- **Multi-Chain Ready**: Built-in support for Sepolia, Base, Arbitrum, Optimism, and more.
- **Scalable Operations**: Designed to handle 10, 50, or 100+ accounts with minimal resource overhead.

## 🛠️ Built With
- **Rust**: For unmatched speed and safety.
- **Ethers-rs**: Professional-grade Ethereum library.
- **Tokio**: Asynchronous runtime for concurrent wallet management.

## 📦 Installation
```bash
git clone https://github.com/padma02-drmn/ghost-army-manager
cd ghost-army-manager
cargo build --release
```

## 🛡️ Security Note
This tool handles private keys. Never commit your `secret_wallet.txt` or `.env` files. This repository includes a pre-configured `.gitignore` to prevent accidental leaks.

---
*Created by: Kadek Padma Darmawan*

## 🔧 Usage

```bash
# Example: show help
cargo run -- --help

# Build release binary
cargo build --release
```

## ⚠️ Disclaimer

This repository is shared for **educational and legitimate testing/automation** purposes.
Do not use it to violate terms of service, abuse networks, or run unauthorized activity.
You are responsible for complying with local laws and the rules of any chain/testnet you interact with.

