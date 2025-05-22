use argon2::{Argon2, Params};
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
use rand::{TryRngCore, rngs::OsRng};

use crate::errors::certificate_error::CertificateError;

pub fn generate_random_salt() -> Result<String, CertificateError> {
    let mut salt = [0u8; 16];

    if let Err(e) = OsRng.try_fill_bytes(&mut salt) {
        return Err(CertificateError::RandomSaltGenerationError(e.to_string()));
    }

    Ok(STANDARD_NO_PAD.encode(&salt))
}

pub fn generate_hash(value: &str, salt: Option<String>) -> Result<String, CertificateError> {
    const MEMORY: u32 = 128 * 1024;
    const ITERATIONS: u32 = 3;
    const PARALLELISM: u32 = 4;
    const OUTPUT_LENGTH: usize = 64;

    let salt_bytes = if let Some(salt_str) = salt {
        STANDARD_NO_PAD
            .decode(salt_str.as_bytes())
            .map_err(|e| CertificateError::Base64DecodeError(format!("{}", e)))?
    } else {
        let new_salt_b64 = generate_random_salt()?;
        STANDARD_NO_PAD
            .decode(new_salt_b64.as_bytes())
            .map_err(|e| CertificateError::Base64DecodeError(format!("{}", e)))?
    };

    let params = Params::new(MEMORY, ITERATIONS, PARALLELISM, None)
        .map_err(|e| CertificateError::HashingError(e.to_string()))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let mut output = vec![0u8; OUTPUT_LENGTH];

    argon2
        .hash_password_into(value.as_bytes(), &salt_bytes, &mut output)
        .map_err(|e| CertificateError::HashingError(e.to_string()))?;

    // Salt + hash'i tek bir base64 stringe çevir
    let mut full = Vec::with_capacity(salt_bytes.len() + output.len());
    full.extend_from_slice(&salt_bytes);
    full.extend_from_slice(&output);

    Ok(STANDARD_NO_PAD.encode(&full))
}
