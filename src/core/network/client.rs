use tbc::errors::network_error::NetworkError;
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};
use crate::{enums::message::MessageType, utils::ip_address::get_global_ip};

pub async fn join(addr: String, port: u16, cert: String) -> Result<(), NetworkError> {
    let mut stream = TcpStream::connect(format!("{}:{}", addr, port))
        .await
        .map_err(|e| NetworkError::ConnectError(e.to_string()))?;

    // JoinRequest
    let message = MessageType::JoinRequest { cert, ip: get_global_ip().await.unwrap() };

    let encoded = bincode::encode_to_vec(&message, bincode::config::standard())
        .map_err(|e| NetworkError::JoinRequestEncodeError(e.to_string()))?;

    // Send request
    stream
        .write_all(&encoded)
        .await
        .map_err(|e| NetworkError::JoinRequestWriteError(e.to_string()))?;

    // Wait for response
    let mut buf = vec![0u8; 1024];
    let n = stream
        .read(&mut buf)
        .await
        .map_err(|e| NetworkError::JoinResponseReadError(e.to_string()))?;

    if n == 0 {
        return Err(NetworkError::JoinResponseReadError("Connection closed".into()));
    }

    let (response, _) = bincode::decode_from_slice::<MessageType, _>(
        &buf[..n],
        bincode::config::standard(),
    )
    .map_err(|e| NetworkError::JoinResponseDecodeError(e.to_string()))?;

    if let MessageType::JoinResponse { accepted: true, reason: None } = response {
        println!("Connected successfully.");
    } else if let MessageType::JoinResponse { accepted: false, reason: Some(reason) } = response {
        println!("Connection rejected: {}", reason);
    }   

    Ok(())
}
