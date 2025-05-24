use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Failed to build request <nw-01>: {0}")]
    ClientBuildError(String),

    #[error("HTTP request failed <nw-02>: {0}")]
    RequestError(String),

    #[error("Failed to read response <nw-03>: {0}")]
    ResponseReadError(String),

    // TCP specific errors

    // Host

    #[error("Failed to bind TCP server <tcp-h-01>: {0}")]
    BindError(String),

    #[error("Failed to accept TCP request <tcp-h-02>: {0}")]
    AcceptError(String),

    #[error("Failed to read TCP request <tcp-h-03>: {0}")]
    JoinRequestReadError(String),

    #[error("Failed to write TCP response <tcp-h-04>: {0}")]
    JoinResponseWriteError(String),

    #[error("Failed to decode TCP request <tcp-h-05>: {0}")]
    JoinRequestDecodeError(String),
    
    #[error("Failed to encode TCP response <tcp-h-06>: {0}")]
    JoinResponseEncodeError(String),

    // Client

    #[error("Failed to connect TCP server <tcp-c-01>: {0}")]
    ConnectError(String),

    #[error("Failed to encode TCP response <tcp-c-02s: {0}")]
    JoinRequestEncodeError(String),

    #[error("Failed to read TCP response <tcp-c-03>: {0}")]
    JoinResponseReadError(String),

    #[error("Failed to decode TCP response <tcp-c-04>: {0}")]
    JoinResponseDecodeError(String),

    #[error("Failed to write TCP request <tcp-c-05>: {0}")]
    JoinRequestWriteError(String),
}