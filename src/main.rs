use ethers::signers::{LocalWallet, Signer};
use ethers::core::rand::thread_rng;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use dotenv::dotenv;
use inquire::{Select, CustomType, Text};
use colored::Colorize;
use comfy_table::{Table, Cell, Color};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    loop {
        // Clear screen logic for a cleaner TUI
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        println!("{}", "==========================================".cyan().bold());
        println!("{}", "   GHOST ARMY MANAGER (GAM-rs) v0.1.0    ".green().bold());
        println!("{}", "        'Silent. Swift. Scalable.'        ".blue().italic());
        println!("{}", "==========================================".cyan().bold());
        println!();

        let options = vec![
            "1. Generate Shadow Wallets",
            "2. Check Army Status",
            "3. Exit",
        ];

        let choice = Select::new("Command Center - Choose your operation:", options)
            .with_page_size(5)
            .prompt();

        match choice {
            Ok(val) => {
                if val.starts_with("1") {
                    generate_wallets()?;
                } else if val.starts_with("2") {
                    check_army_status()?;
                } else {
                    println!("{}", "Commander out. Shutting down system...".yellow());
                    break;
                }
            }
            Err(_) => {
                println!("{}", "Error parsing selection or operation cancelled. Exiting.".red());
                break;
            }
        }

        // Wait for user before looping again
        println!("\n------------------------------------------");
        let _ = Text::new("Press Enter to continue...").prompt();
    }

    Ok(())
}

fn generate_wallets() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "--- GENERATING SHADOW ARMY ---".magenta().bold());
    
    let count: u32 = CustomType::<u32>::new("How many shadow wallets do you want to generate?")
        .with_default(10)
        .with_error_message("Please enter a valid positive integer.")
        .prompt()?;

    if count == 0 {
        println!("{}", "Operation cancelled. Target count must be greater than 0.".yellow());
        return Ok(());
    }

    // Determine starting ID by reading file if it exists
    let mut starting_id = 1;
    if let Ok(file) = File::open("secret_wallet.txt") {
        let reader = BufReader::new(file);
        let lines_count = reader.lines().count();
        starting_id = lines_count + 1;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("secret_wallet.txt")?;

    for i in 0..count {
        let wallet = LocalWallet::new(&mut thread_rng());
        let address = format!("{:?}", wallet.address());
        let priv_key = format!("0x{}", hex::encode(wallet.signer().to_bytes()));

        let current_id = starting_id + i as usize;

        writeln!(file, "ID: {} | ADDRESS: {} | KEY: {}", current_id, address, priv_key)?;
        println!("Shadow Wallet #{} Ready: {}", current_id.to_string().cyan(), address.green());
    }

    println!("------------------------------------------");
    println!("{} {} {}", "✅ MISSION SUCCESS:".green().bold(), count.to_string().yellow(), "Shadow Wallets added to secret_wallet.txt".green());
    println!("{}", "⚠️  Check .gitignore: secret_wallet.txt is safely excluded.".yellow());
    println!("{}", "Smith: Pasukan bayangan sudah masuk barak, Padma! Siap tunggu komando.".blue().italic());

    Ok(())
}

fn check_army_status() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "--- CHECKING ARMY STATUS ---".magenta().bold());

    let file = match File::open("secret_wallet.txt") {
        Ok(f) => f,
        Err(_) => {
            println!("{}", "No shadow army found. Please generate wallets first (secret_wallet.txt not found).".red());
            return Ok(());
        }
    };

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").fg(Color::Cyan),
        Cell::new("Address").fg(Color::Green),
        Cell::new("Private Key (MASKED)").fg(Color::Red),
    ]);

    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        // Format: ID: 1 | ADDRESS: 0x... | KEY: 0x...
        let parts: Vec<&str> = line.split(" | ").collect();
        if parts.len() == 3 {
            let id_part = parts[0].replace("ID: ", "");
            let addr_part = parts[1].replace("ADDRESS: ", "");
            // Mask the key
            let key_part = "******************************************************************";

            table.add_row(vec![
                id_part,
                addr_part,
                key_part.to_string(),
            ]);
            count += 1;
        }
    }

    if count > 0 {
        println!("{table}");
        println!("{} {} {}", "Total active shadow units:".blue(), count.to_string().yellow().bold(), "units.".blue());
    } else {
        println!("{}", "secret_wallet.txt is empty. No shadow army found.".yellow());
    }

    Ok(())
}
