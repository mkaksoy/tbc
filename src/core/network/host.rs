use tbc::errors::network_error::NetworkError;
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener};

use crate::enums::message::MessageType;
use crate::enums::message::MessageType::*;

pub async fn create(port: u16, host_cert: String) -> Result<(), NetworkError> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .map_err(|e| NetworkError::BindError(e.to_string()))?;

    loop {
        // Accept the connection
        let (mut stream, _) = listener
            .accept()
            .await
            .map_err(|e| NetworkError::AcceptError(e.to_string()))?;

        let host_cert = host_cert.clone();
        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];

            // Wait for join request
            loop {
                let message = match stream.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(_) => break,
                };

                let decoded = match bincode::decode_from_slice::<MessageType, _>(
                    &buf[..message],
                    bincode::config::standard(),
                ) {
                    Ok(val) => val,
                    Err(_) => {
                        break;
                    }
                };

                match decoded.0 {
                    JoinRequest { cert, ip } => {
                        if !host_cert.contains(cert.as_str()) {
                            let response = JoinResponse {
                                accepted: false,
                                reason: Some("Invalid certificate".to_string()),
                            };

                            let encoded =
                                bincode::encode_to_vec(&response, bincode::config::standard())
                                    .map_err(|e| NetworkError::JoinResponseEncodeError(e.to_string()))
                                    .unwrap();

                            stream
                                .write_all(&encoded)
                                .await
                                .map_err(|e| NetworkError::JoinResponseWriteError(e.to_string()))
                                .unwrap();
                            continue;
                        }
                        let response = JoinResponse {
                            accepted: true,
                            reason: None,
                        };

                        let encoded =
                            bincode::encode_to_vec(&response, bincode::config::standard())
                                .map_err(|e| NetworkError::JoinResponseEncodeError(e.to_string()))
                                .unwrap();

                        stream
                            .write_all(&encoded)
                            .await
                            .map_err(|e| NetworkError::JoinResponseWriteError(e.to_string()))
                            .unwrap();

                        println!("User with ip {} has joined.", ip);

                        // TODO : add the user to the list
                    }
                    _ => {
                        continue;
                    }
                }
            }
        });
    }
}
