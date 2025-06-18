use crate::formatter::format_messages;
use crate::message::MessageResponse;
use anyhow::Result;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Represents a client that connects to the RAC server.
#[derive(Debug, Default)]
pub struct Client {
    pub messages_count: usize,
    user_name: Option<String>,
    address: Option<String>,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Connection error occurred")]
    ConnectionError,

    #[error("Address not set for the client")]
    AddressNotSet,

    #[error("Error writing to the stream")]
    StreamWriteError,

    #[error("Error reading from the stream")]
    StreamReadError,

    #[error("Failed to parse message count from response: {0}")]
    ParseError(String),
}

impl Client {
    pub fn new() -> Self {
        Self {
            messages_count: 0,
            user_name: None,
            address: None,
        }
    }

    /// Prepares the TCP stream for communication with RAC server.
    pub async fn get_stream(&self) -> Result<TcpStream, ClientError> {
        if let Some(addr) = &self.address {
            TcpStream::connect(addr)
                .await
                .map_err(|_| ClientError::ConnectionError)
        } else {
            Err(ClientError::AddressNotSet)
        }
    }

    /// Sets up the connection by connecting to the given address and initializing the username.
    pub async fn setup_connection(
        &mut self,
        address: &str,
        user_name: &str,
    ) -> Result<(), ClientError> {
        // Testing connection here.
        self.address = Some(address.to_string());
        let _ = self.get_stream().await?;
        
        let count = self.fetch_messages_count().await?;
        self.messages_count = count as usize;
        self.user_name = Some(user_name.to_string());
        Ok(())
    }

    /// Disconnects the client by clearing the address, messages count, and username.
    pub async fn disconnect(&mut self) {
        self.address = None;
        self.messages_count = 0;
        self.user_name = None;
    }

    /// Sends a message to the server.
    pub async fn send_message(&self, message: &str) -> Result<(), ClientError> {
        let mut stream = self.get_stream().await?;
        let formatted_message =
            format!("\u{25B2}<{}> {}", self.user_name.clone().unwrap(), message);
        stream
            .write_all(format!("\x01{}", formatted_message).as_bytes())
            .await
            .map_err(|_| ClientError::StreamWriteError)?;
        Ok(())
    }

    /// Fetches the count of messages from the server.
    pub async fn fetch_messages_count(&self) -> Result<u64, ClientError> {
        let mut stream = self.get_stream().await?;
        stream
            .write_all(&[0x00])
            .await
            .map_err(|_| ClientError::StreamWriteError)?;
        let mut buffer = [0u8; 1024];
        let n = stream
            .read(&mut buffer)
            .await
            .map_err(|_| ClientError::StreamReadError)?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        let count = response.parse::<u64>().map_err(|_| {
            ClientError::ParseError("Failed to parse message count from response".to_string())
        })?;
        Ok(count)
    }

    /// Retrieves new messages from the server since the last fetch.
    pub async fn get_messages(&mut self) -> Result<Vec<MessageResponse>, ClientError> {
        let mut stream = self.get_stream().await?;

        // Requesting messages count
        stream
            .write_all(&[0x00])
            .await
            .map_err(|_| ClientError::StreamWriteError)?;
        let mut head = [0u8; 1024];
        let n = stream
            .read(&mut head)
            .await
            .map_err(|_| ClientError::StreamReadError)?;
        let count = String::from_utf8_lossy(&head[..n])
            .parse::<usize>()
            .map_err(|_| {
                ClientError::ParseError("Failed to parse message count from response".to_string())
            })?;

        if count <= self.messages_count {
            return Ok(Vec::new());
        }

        // Requesting new messages
        stream
            .write_all(format!("\x02{}", self.messages_count).as_bytes())
            .await
            .map_err(|_| ClientError::StreamWriteError)?;
        let mut buffer = vec![0u8; count - self.messages_count];
        stream
            .read_exact(&mut buffer)
            .await
            .map_err(|_| ClientError::StreamReadError)?;
        let response = String::from_utf8_lossy(&buffer).into_owned();

        let vec_messages = response
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();
        let messages: Vec<MessageResponse> = format_messages(vec_messages);

        self.messages_count = count;

        Ok(messages)
    }
}
