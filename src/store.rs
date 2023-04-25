use anyhow::*;
use std::collections::HashMap;
use std::env;
use std::env::consts::OS;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

const STORE_TEMPLATE: &str = "\
# OVA store file
# Expected values like
# KEY = VALUE
# You can add keys manually
# or use the 'ova add <name> <key>' command\n";

const STORE_FILE_NAME: &str = "ova.store";

#[cfg(target_os = "linux")]
const STORE_DIR_NAME: &str = ".config";

#[cfg(target_os = "macos")]
const STORE_DIR_NAME: &str = ".config";

#[cfg(target_os = "windows")]
const STORE_DIR_NAME: &str = "AppData/Local";

pub struct Store {
    pub keys: HashMap<String, String>,
    file_path: PathBuf,
}

impl Store {
    pub fn new() -> Result<Self> {
        let home_dir = match OS {
            "linux" => env::var("HOME").with_context(|| "Error getting HOME env variable")?,
            "macos" => env::var("HOME").with_context(|| "Error getting HOME env variable")?,
            "windows" => {
                env::var("USERPROFILE").with_context(|| "Error getting USERPROFILE env variable")?
            }
            _ => return Err(anyhow!("Unsupported OS")),
        };

        let path: PathBuf = PathBuf::new()
            .join(home_dir)
            .join(STORE_DIR_NAME)
            .join(STORE_FILE_NAME);

        if !path.exists() {
            Self::create_store_file(&path).with_context(|| {
                format!("Error creating config file at - {}", path.to_string_lossy())
            })?;
        }

        Ok(Store {
            keys: Self::read_store_file(&path).with_context(|| {
                format!("Error reading config file at - {}", path.to_string_lossy())
            })?,
            file_path: path,
        })
    }

    fn read_store_file(path: &PathBuf) -> Result<HashMap<String, String>> {
        let file_content = Self::read_store_file_as_string(path)?;

        let keys_map: HashMap<String, String> = file_content
            .into_iter()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .filter_map(|line| {
                let mut split = line.splitn(2, '=');
                let key = split.next()?.trim().to_owned();
                let value = split.next()?.trim().to_owned();

                Some((key, value))
            })
            .collect();

        Ok(keys_map)
    }

    fn read_store_file_as_string(path: &PathBuf) -> Result<Vec<String>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut file_content: Vec<String> = vec![];

        for line in reader.lines() {
            let line = line.unwrap();
            if !line.is_empty() && !line.starts_with("#") {
                file_content.push(line);
            }
        }

        Ok(file_content)
    }

    fn create_store_file(path: &PathBuf) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(STORE_TEMPLATE.as_bytes())?;
        Ok(())
    }

    pub fn insert_into_store(&self, key: &String, value: &String) -> Result<()> {
        let mut file = OpenOptions::new().append(true).open(&self.file_path)?;
        file.write_all(format!("{} = {}\n", key, value).as_bytes())?;
        Ok(())
    }
}
