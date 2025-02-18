use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use crate::{encryption::{encrypt, decrypt}, error::VaultError};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordVault {
    entries: HashMap<String, PasswordEntry>,
}

impl PasswordVault {
    pub fn new() -> Self {
        PasswordVault {
            entries: HashMap::new(),
        }
    }

    pub fn load(path: &Path, master_password: &str) -> Result<Self, VaultError> {
        if path.exists() {
            let encrypted_data = fs::read(path)?;
            let json_data = decrypt(&encrypted_data, master_password)
                .map_err(|_| VaultError::InvalidPassword)?;
            Ok(serde_json::from_slice(&json_data)?)
        } else {
            let vault = Self::new();
            let _ = vault.save(path, master_password);
            Ok(vault)
        }
    }
    
    pub fn save(&self, path: &Path, master_password: &str) -> Result<(), VaultError> {
        let json_data = serde_json::to_vec(self)?;
        let encrypted_data = encrypt(&json_data, master_password)?;
        fs::write(path, encrypted_data)?;
        Ok(())
    }

    pub fn add_entry(&mut self, website: &str, username: &str, password: &str) {
        self.entries.insert(
            website.to_string(),
            PasswordEntry {
                username: username.to_string(),
                password: password.to_string(),
            },
        );
    }

    pub fn get_entry(&self, website: &str) -> Option<&PasswordEntry> {
        self.entries.get(website)
    }
}
