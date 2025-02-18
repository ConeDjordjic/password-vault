use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Encryption error: {0}")]
    Encryption(#[from] aes_gcm::Error),
    
    #[error("Argon2 error: {0}")]
    Argon2(String),
    
    #[error("Hex encoding error: {0}")]
    Hex(#[from] hex::FromHexError),
    
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error("Invalid master password")]
    InvalidPassword,
}

impl From<argon2::password_hash::Error> for VaultError {
    fn from(err: argon2::password_hash::Error) -> Self {
        VaultError::Argon2(err.to_string())
    }
}
