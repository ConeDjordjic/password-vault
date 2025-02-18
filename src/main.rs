mod error;
mod vault;
mod encryption;
mod password;

use clap::Parser;
use error::VaultError;
use std::path::PathBuf;
use vault::PasswordVault;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(short, long, default_value = "vault.enc")]
    vault_file: PathBuf,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Add {
        website: String,
        username: String,
        #[clap(short, long)]
        length: Option<usize>,
        #[clap(short, long)]
        special: bool,
    },
    Get {
        website: String,
    },
    Generate {
        #[clap(short, long)]
        length: Option<usize>,
        #[clap(short, long)]
        special: bool,
    },
}

fn main() -> Result<(), VaultError> {
    let args = Args::parse();
    let master_password = rpassword::prompt_password("Master password: ")?;

    match args.command {
        Command::Add {
            website,
            username,
            length,
            special,
        } => {
            let mut vault = PasswordVault::load(&args.vault_file, &master_password)?;
            let password = password::generate_password(length.unwrap_or(16), special);
            vault.add_entry(&website, &username, &password);
            vault.save(&args.vault_file, &master_password)?;
            println!("Added credentials for {}", website);
        }
        Command::Get { website } => {
            let vault = PasswordVault::load(&args.vault_file, &master_password)?;
            if let Some(entry) = vault.get_entry(&website) {
                println!("Username: {}\nPassword: {}", entry.username, entry.password);
            } else {
                println!("No entry found for {}", website);
            }
        }
        Command::Generate { length, special } => {
            let password = password::generate_password(length.unwrap_or(16), special);
            println!("Generated password: {}", password);
        }
    }

    Ok(())
}
