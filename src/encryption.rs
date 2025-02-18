use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
    Aes256Gcm, Key
};
use argon2::{password_hash::{SaltString, PasswordHasher, PasswordHash}};
use argon2::Argon2;
use rand::RngCore;

const SALT_LENGTH: usize = 16;
const NONCE_LENGTH: usize = 12;

pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>, aes_gcm::Error> {
    let mut salt = [0u8; SALT_LENGTH];
    OsRng.fill_bytes(&mut salt);
    
    let salt_string = SaltString::encode_b64(&salt).unwrap();
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt_string).unwrap();
    let hash_obj = hash.hash.unwrap();
    let key = hash_obj.as_bytes();

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    let mut buffer = data.to_vec();
    cipher.encrypt_in_place(&nonce, b"", &mut buffer)?;
    
    let mut result = Vec::with_capacity(SALT_LENGTH + NONCE_LENGTH + buffer.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce);
    result.extend(buffer);
    Ok(result)
}

pub fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>, aes_gcm::Error> {
    if data.len() < SALT_LENGTH + NONCE_LENGTH {
        return Err(aes_gcm::Error);
    }
    
    let salt = &data[..SALT_LENGTH];
    let nonce = &data[SALT_LENGTH..SALT_LENGTH + NONCE_LENGTH];
    let ciphertext = &data[SALT_LENGTH + NONCE_LENGTH..];
    
    let salt_string = SaltString::encode_b64(salt).unwrap();
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt_string).unwrap();
    let hash_string = hash.to_string();
    let parsed_hash = PasswordHash::new(&hash_string).unwrap();
    let hash_obj = parsed_hash.hash.unwrap();
    let key = hash_obj.as_bytes();

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut buffer = ciphertext.to_vec();
    cipher.decrypt_in_place(nonce.into(), b"", &mut buffer)?;
    
    Ok(buffer)
}
