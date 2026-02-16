use ethers::signers::{LocalWallet, Signer};
use ethers::core::rand::thread_rng;
use std::fs::OpenOptions;
use std::io::Write;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("==========================================");
    println!("   GHOST ARMY MANAGER (GAM-rs) v0.1.0    ");
    println!("        'Silent. Swift. Scalable.'        ");
    println!("==========================================");

    // Command menu (basic version)
    println!("1. Generate 10 Shadow Wallets");
    println!("2. Check Army Status (Coming Soon)");
    println!("------------------------------------------");

    // Execution logic for option 1
    println!("--- GENERATING SHADOW ARMY ---");
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("secret_wallet.txt")?;

    for i in 1..=10 {
        let wallet = LocalWallet::new(&mut thread_rng());
        let address = format!("{:?}", wallet.address());
        let priv_key = format!("0x{}", hex::encode(wallet.signer().to_bytes()));

        writeln!(file, "ID: {} | ADDRESS: {} | KEY: {}", i, address, priv_key)?;
        println!("Shadow Wallet #{} Ready: {}", i, address);
    }

    println!("------------------------------------------");
    println!("✅ MISSION SUCCESS: 10 Shadow Wallets added to secret_wallet.txt");
    println!("⚠️  Check .gitignore: secret_wallet.txt is safely excluded.");
    println!("Smith: Pasukan bayangan sudah masuk barak, Padma! Siap tunggu komando.");

    Ok(())
}
