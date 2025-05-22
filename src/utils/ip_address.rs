use reqwest::blocking::Client;

use crate::errors::network_error::NetworkError;

pub fn get_global_ip() -> Result<String, NetworkError> {
    let client = Client::builder()
        .build()
        .map_err(|e| NetworkError::ClientBuildError(e.to_string()))?;

    let resp = client
        .get("https://checkip.amazonaws.com/")
        .send()
        .map_err(|e| NetworkError::RequestError(e.to_string()))?;

    let ip = resp
        .text()
        .map_err(|e| NetworkError::ResponseReadError(e.to_string()))?
        .trim()
        .to_string();

    Ok(ip)
}
