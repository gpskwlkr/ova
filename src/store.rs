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
            if !line.is_empty() && !line.starts_with('#') {
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
        if key.trim().is_empty() || value.trim().is_empty() {
            return Err(anyhow!("Key cannot be empty"));
        }

        if value.trim().len() < 16 {
            return Err(anyhow!("Key must be at least 16 characters long"));
        }

        if self.keys.contains_key(key) {
            return Err(anyhow!("Key already exists"));
        }

        let mut file = OpenOptions::new().append(true).open(&self.file_path)?;

        file.write_all(format!("{} = {}\n", key, value).as_bytes())?;
        Ok(())
    }

    pub fn delete_from_store(&self, key: &String) -> Result<()> {
        if key.trim().is_empty() {
            return Err(anyhow!("Key cannot be empty"));
        }

        let (index_to_remove, mut file_content) = Self::find_line_index(key, &self.file_path)?;

        if let Some(index) = index_to_remove {
            file_content.remove(index);
        }

        let mut file = File::create(&self.file_path)?;
        file.write_all(file_content.join("\n").as_bytes())?;
        Ok(())
    }

    pub fn update_store(&self, key: &String, value: &String) -> Result<()> {
        if key.trim().is_empty() || value.trim().is_empty() {
            return Err(anyhow!("Key cannot be empty"));
        }

        if value.trim().len() < 16 {
            return Err(anyhow!("Key must be at least 16 characters long"));
        }

        let (index_to_update, mut file_content) = Self::find_line_index(key, &self.file_path)?;

        if let Some(index) = index_to_update {
            file_content[index] = format!("{} = {}", key, value);
        }

        let mut file = File::create(&self.file_path)?;
        file.write_all(file_content.join("\n").as_bytes())?;
        Ok(())
    }

    fn find_line_index(key: &String, path: &PathBuf) -> Result<(Option<usize>, Vec<String>)> {
        let file_content = Self::read_store_file_as_string(path)?;
        let mut index: Option<usize> = None;
        for (i, line) in file_content.iter().enumerate() {
            if line.starts_with(key) {
                index = Some(i);
                break;
            }
        }

        Ok((index, file_content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::path::PathBuf;

    #[test]
    fn test_create_store_file() {
        let path = PathBuf::new().join("test_create.store");

        let result = Store::create_store_file(&path);
        assert!(result.is_ok());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_read_store_file_as_string() {
        let path = PathBuf::new().join("test_read_as_string.store");
        let _ = File::create(&path);

        let key = "test_key".to_string();
        let value = "1234567890123456".to_string();

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&path)
            .unwrap();
        file.write_all(format!("{} = {}", key, value).as_bytes())
            .unwrap();

        let file_content = Store::read_store_file_as_string(&path).unwrap();
        assert_eq!(file_content.len(), 1);
        assert_eq!(file_content[0], "test_key = 1234567890123456");

        remove_file(path).unwrap();
    }

    #[test]
    fn test_read_store_file_as_string_with_empty_file() {
        let path = PathBuf::new().join("test_read_empty.store");
        let _ = File::create(&path);

        let file_content = Store::read_store_file_as_string(&path).unwrap();
        assert_eq!(file_content.len(), 0);

        remove_file(path).unwrap();
    }

    #[test]
    fn test_read_store_file_as_string_with_non_existing_file() {
        let path = PathBuf::new().join("test_read_non_existing.store");

        let result = Store::read_store_file_as_string(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_store_file() {
        let path = PathBuf::new().join("test_read.store");
        let _ = File::create(&path);

        let key = "test_key".to_string();
        let value = "1234567890123456".to_string();

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&path)
            .unwrap();
        file.write_all(format!("{} = {}", key, value).as_bytes())
            .unwrap();

        let file_content = Store::read_store_file(&path).unwrap();

        file_content.iter().for_each(|entry| {
            assert_eq!(entry.0, "test_key");
            assert_eq!(entry.1, "1234567890123456");
        });

        remove_file(path).unwrap();
    }

    #[test]
    fn test_insert_into_store() {
        let path = PathBuf::new().join("test.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "test_key".to_string();
        let value = "abcdefghijklmnop".to_string();

        store.insert_into_store(&key, &value).unwrap();

        let file_content = Store::read_store_file_as_string(&path).unwrap();
        assert_eq!(file_content.len(), 1);
        assert_eq!(file_content[0], "test_key = abcdefghijklmnop");

        remove_file(path).unwrap();
    }

    #[test]
    fn test_insert_into_store_with_empty_key() {
        let path = PathBuf::new().join("test_empty.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "".to_string();
        let value = "test_value".to_string();

        let result = store.insert_into_store(&key, &value);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_insert_into_store_with_empty_value() {
        let path = PathBuf::new().join("test_empty_value.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "test_key".to_string();
        let value = "".to_string();

        let result = store.insert_into_store(&key, &value);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_insert_into_store_with_empty_key_and_value() {
        let path = PathBuf::new().join("test_empty_key_value.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "".to_string();
        let value = "".to_string();

        let result = store.insert_into_store(&key, &value);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_insert_into_store_with_key_less_than_16_chars() {
        let path = PathBuf::new().join("test_key_less_than_16_chars.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "abcdefghijklmnop".to_string();
        let value = "abcdefghijklmno".to_string();

        let result = store.insert_into_store(&key, &value);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_insert_into_store_with_already_existing_key() {
        let path = PathBuf::new().join("test_existing_key.store");
        let _ = File::create(&path);
        let mut store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "test_key".to_string();
        let value = "abcdefghijklmnop".to_string();

        store.insert_into_store(&key, &value).unwrap();
        store.keys.insert(key.clone(), value.clone());
        let result = store.insert_into_store(&key, &value);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_from_store() {
        let path = PathBuf::new().join("test_delete.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "test_key".to_string();
        let value = "1234567890123456".to_string();

        store.insert_into_store(&key, &value).unwrap();
        store.delete_from_store(&key).unwrap();

        let file_content = Store::read_store_file_as_string(&path).unwrap();
        assert_eq!(file_content.len(), 0);

        remove_file(path).unwrap();
    }

    #[test]
    fn test_delete_from_store_with_empty_key() {
        let path = PathBuf::new().join("test_delete_empty_key.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "".to_string();

        let result = store.delete_from_store(&key);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_update_store() {
        let path = PathBuf::new().join("test_update.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "test_key".to_string();
        let value = "1234567890123456".to_string();

        store.insert_into_store(&key, &value).unwrap();
        store
            .update_store(&key, &"1234567890123456".to_string())
            .unwrap();

        let file_content = Store::read_store_file_as_string(&path).unwrap();
        assert_eq!(file_content.len(), 1);
        assert_eq!(file_content[0], "test_key = 1234567890123456");

        remove_file(path).unwrap();
    }

    #[test]
    fn test_update_store_with_empty_key() {
        let path = PathBuf::new().join("test_update_empty_key.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "".to_string();
        let value = "1234567890123456".to_string();

        let result = store.update_store(&key, &value);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_update_store_with_empty_value() {
        let path = PathBuf::new().join("test_update_empty_value.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "test_key".to_string();
        let value = "".to_string();

        let result = store.update_store(&key, &value);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_update_store_with_empty_key_and_value() {
        let path = PathBuf::new().join("test_update_empty_key_value.store");
        let _ = File::create(&path);
        let store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "".to_string();
        let value = "".to_string();

        let result = store.update_store(&key, &value);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_update_store_with_key_less_than_16_chars() {
        let path = PathBuf::new().join("test_update_key_less_than_16_chars.store");
        let _ = File::create(&path);
        let mut store = Store {
            keys: HashMap::new(),
            file_path: path.clone(),
        };

        let key = "abcdefghijklmnop".to_string();
        let value = "abcdefghijklmno".to_string();
        let insert_value = "abcdefghijklmnop".to_string();

        store.insert_into_store(&key, &insert_value).unwrap();
        store.keys.insert(key.clone(), insert_value.clone());
        let result = store.update_store(&key, &value);
        assert!(result.is_err());

        remove_file(path).unwrap();
    }
}
