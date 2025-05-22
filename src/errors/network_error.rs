use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Failed to build request <nw01>: {0}")]
    ClientBuildError(String),

    #[error("HTTP request failed <nw02>: {0}")]
    RequestError(String),

    #[error("Failed to read response <nw03>: {0}")]
    ResponseReadError(String),
}