use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::SaltString,
    Argon2,
};
use base64::{engine::general_purpose::STANDARD as BASE64_ENGINE, Engine as _};
use rand_core::{OsRng, RngCore};

// Define constants
const KEY_LEN: usize = 32; // 256 bits for AES-256
const NONCE_LEN: usize = 12; // 96 bits for AES-GCM

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Argon2 password hashing error: {0}")]
    Argon2PasswordHash(argon2::password_hash::Error),

    #[error("Argon2 library error: {0}")]
    Argon2Lib(argon2::Error),

    #[error("AES-GCM encryption/decryption failed")]
    Aes,

    #[error("Base64 decoding failed: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Invalid encrypted data format")]
    Format,

    #[error("UTF-8 conversion failed: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

// Custom result type for our crypto functions
pub type CryptoResult<T> = Result<T, String>;

// Wrapper to convert our specific error into a String for Tauri
fn to_tauri_error<T>(res: Result<T, CryptoError>) -> CryptoResult<T> {
    res.map_err(|e| e.to_string())
}

pub fn encrypt(data: &str, password: &str) -> CryptoResult<String> {
    to_tauri_error(_encrypt(data, password))
}

pub fn decrypt(encoded_data: &str, password: &str) -> CryptoResult<String> {
    to_tauri_error(_decrypt(encoded_data, password))
}

fn _encrypt(data: &str, password: &str) -> Result<String, CryptoError> {
    let salt = SaltString::generate(&mut OsRng);
    let key = derive_key(password.as_bytes(), &salt)?;

    let cipher = Aes256Gcm::new(&key.into());

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, data.as_bytes()).map_err(|_| CryptoError::Aes)?;

    // Combine salt, nonce, and ciphertext with a separator
    let mut result = Vec::new();
    result.extend_from_slice(salt.as_str().as_bytes());
    result.extend_from_slice(b"|");
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(b"|");
    result.extend_from_slice(&ciphertext);

    Ok(BASE64_ENGINE.encode(&result))
}

fn _decrypt(encoded_data: &str, password: &str) -> Result<String, CryptoError> {
    let decoded_bytes = BASE64_ENGINE.decode(encoded_data)?;

    let parts: Vec<&[u8]> = decoded_bytes.split(|&b| b == b'|').collect();
    if parts.len() != 3 {
        return Err(CryptoError::Format);
    }

    let salt_str = std::str::from_utf8(parts[0]).map_err(|_| CryptoError::Format)?;
    let nonce_bytes = parts[1];
    let ciphertext = parts[2];

    if nonce_bytes.len() != NONCE_LEN {
        return Err(CryptoError::Format);
    }

    let salt = SaltString::new(salt_str).map_err(CryptoError::Argon2PasswordHash)?;
    let key = derive_key(password.as_bytes(), &salt)?;

    let cipher = Aes256Gcm::new(&key.into());
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext_bytes = cipher.decrypt(nonce, ciphertext).map_err(|_| CryptoError::Aes)?;

    Ok(String::from_utf8(plaintext_bytes)?)
}

fn derive_key(password: &[u8], salt: &SaltString) -> Result<[u8; KEY_LEN], CryptoError> {
    let mut key = [0u8; KEY_LEN];
    // The salt must be passed as a byte slice, and the error must be mapped.
    Argon2::default()
        .hash_password_into(password, salt.as_ref().as_bytes(), &mut key)
        .map_err(CryptoError::Argon2Lib)?;
    Ok(key)
}

