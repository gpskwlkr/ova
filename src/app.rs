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

        #[clap(long, short = 'c')]
        /// Copy the key to the clipboard
        copy: Option<bool>,
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
            Command::Get { name, copy } => {
                let key = match store.keys.get(name) {
                    Some(key) => key,
                    None => {
                        return Err(anyhow!("Key not found"));
                    }
                };

                let code = get_2fa_code(key)?;

                if copy.is_some() {
                    let mut clip = crate::clip::Clip::new()?;
                    clip.copy(&code)?;
                    println!("Copied code for {} to clipboard", name);
                } else {
                    println!("Key: {}", code);
                }
            }

            Command::Add { name, key } => {
                store.insert_into_store(name, key)?;
                println!("Added {} with key {}", name, key);
            }
            Command::Remove { name } => {
                store.delete_from_store(name)?;
                println!("Removed {}", name);
            }
            Command::List => {
                println!("Key\tValue");
                println!("-----\t-----");
                for (name, key) in store.keys.iter() {
                    println!("{}\t{}", name, key);
                }
            }

            Command::Update { name, key } => {
                store.update_store(name, key)?;
                println!("Updated {} with key {}", name, key);
            }
        }

        Ok(())
    }
}
