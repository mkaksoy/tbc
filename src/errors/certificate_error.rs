use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum CertificateError {
    #[error("Error generating salt <ce01>: {0}")]
    RandomSaltGenerationError(String),

    #[error("Error hashing the password <ce02>: {0}")]
    HashingError(String),
    
    #[error("Error decoding Base64 <ce03>: {0}")]
    Base64DecodeError(String),

    #[error("Error encoding the salt <ce04>: {0}")]
    EncodingSaltError(String),

    #[error("Error encoding the hash <ce05>: {0}")]
    EncodingHashError(String),
}