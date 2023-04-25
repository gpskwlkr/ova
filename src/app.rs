use crate::{store::Store, utils::get_2fa_code};
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
    #[clap(alias = "g")]
    #[clap(about = "Get a key")]
    Get {
        #[clap(long, short = 'n')]
        /// Name of the key (e.g. github)
        name: String,
    },

    #[clap(alias = "a")]
    #[clap(about = "Add a new key")]
    Add {
        #[clap(long, short = 'n')]
        /// Name of the key (e.g. github)
        name: String,

        #[clap(long, short = 'k')]
        /// Key value (e.g. 1234567890)
        key: String,
    },

    #[clap(alias = "r")]
    #[clap(about = "Remove a key")]
    Remove {
        #[clap(long, short = 'n')]
        /// Name of the key to remove
        name: String,
    },

    #[clap(alias = "l")]
    #[clap(about = "List all keys")]
    List,

    #[clap(alias = "u")]
    #[clap(about = "Update a key")]
    Update {
        #[clap(long, short = 'n')]
        /// Name of the key to update
        name: String,

        #[clap(long, short = 'k')]
        /// New key value
        key: String,
    },
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    pub fn new() -> Self {
        App::parse()
    }

    pub fn run(&self) -> Result<()> {
        let store = Store::new()?;
        match &self.command {
            Command::Get { name } => {
                let key = match store.keys.get(name) {
                    Some(key) => key,
                    None => {
                        return Err(anyhow!("Key not found"));
                    }
                };
                println!("Key: {}", get_2fa_code(key)?);
            }

            Command::Add { name, key } => {
                if key.is_empty() {
                    return Err(anyhow!("Key cannot be empty"));
                } else if key.len() < 16 {
                    return Err(anyhow!("Key must be at least 16 characters long"));
                }

                println!("Adding {} with key {}", name, key);
                store.insert_into_store(name, key)?;
            }
            Command::Remove { name } => {
                println!("Removing {}", name);
            }
            Command::List => {
                println!("Key\tValue");
                println!("-----\t-----");
                for (name, key) in store.keys.iter() {
                    println!("{}\t{}", name, key);
                }
            }

            Command::Update { name, key } => {
                println!("Updating {} with key {}", name, key);
            }
        }

        Ok(())
    }
}
