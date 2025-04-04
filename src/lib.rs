use serde::{Deserialize, Serialize};
use reqwest::{Client, Error};
use tokio::sync::mpsc;
use tokio::time::Duration;
use serde_json::json;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize)]
struct SamResponse {
    success: bool,
    message: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SamRequest {
    request: String,
    options: Option<serde_json::Value>,
}

pub struct I2PSamClient {
    client: Client,
    sam_host: String,
    sam_port: u16,
}

impl I2PSamClient {
    // Constructor to initialize the SAM client
    pub fn new(sam_host: String, sam_port: u16) -> I2PSamClient {
        I2PSamClient {
            client: Client::new(),
            sam_host,
            sam_port,
        }
    }

    // Send a request to SAM and get the response
    async fn send_request(&self, request: &str, options: Option<serde_json::Value>) -> Result<SamResponse, Error> {
        let url = format!("http://{}:{}/i2psam", self.sam_host, self.sam_port);
        let sam_request = SamRequest {
            request: request.to_string(),
            options,
        };

        let response = self
            .client
            .post(&url)
            .json(&sam_request)
            .send()
            .await?;

        let sam_response: SamResponse = response.json().await?;

        Ok(sam_response)
    }

    // Open an I2P session (e.g., to create a destination)
    pub async fn create_session(&self) -> Result<String> {
        let options = json!({
            "session": "create",
            "destination": "true",
        });

        let response = self.send_request("session", Some(options)).await?;

        if response.success {
            return Ok(response.message.unwrap_or_else(|| "Unknown destination".to_string()))
        } else {
            return Err(anyhow::anyhow!("Error creating session: {}", response.error.unwrap_or_else(|| "Unknown error".to_string())));
        }
    }

    // Close an I2P session
    pub async fn close_session(&self, session_id: &str) -> Result<()> {
        let options = json!({
            "session": session_id
        });

        let response = self.send_request("close", Some(options)).await?;

        if response.success {
            Ok(())
        } else {
            return Err(anyhow::anyhow!("Error creating session: {}", response.error.unwrap_or_else(|| "Unknown error".to_string())));
        }
    }

    // Send a message to an I2P destination
    pub async fn send_message(&self, destination: &str, message: &str) -> Result<()> {
        let options = json!({
            "destination": destination,
            "message": message,
        });

        let response = self.send_request("sendmsg", Some(options)).await?;

        if response.success {
            Ok(())
        } else {
            return Err(anyhow::anyhow!("Error creating session: {}", response.error.unwrap_or_else(|| "Unknown error".to_string())));
        }
    }

    // Receive a message from an I2P destination
    pub async fn receive_message(&self, session_id: &str) -> Result<String> {
        let options = json!({
            "session": session_id
        });

        let response = self.send_request("recvmsg", Some(options)).await?;

        if response.success {
            Ok(response.message.unwrap_or_else(|| "No message".to_string()))
        } else {
            return Err(anyhow::anyhow!("Error creating session: {}", response.error.unwrap_or_else(|| "Unknown error".to_string())));
        }
    }
}
