use crate::formatter::format_messages;
use crate::message::MessageResponse;
use crate::results::ConnectionResult;
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug, Default)]
pub struct Server {
    pub messages_count: usize,
    user_name: Option<String>,
    address: Option<String>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            messages_count: 0,
            user_name: None,
            address: None,
        }
    }

    pub async fn setup_connection(
        &mut self,
        address: &str,
        user_name: &str,
    ) -> Result<ConnectionResult> {
        println!("Attempting to connect to {}", address);

        match TcpStream::connect(address).await {
            Ok(_) => {
                self.address = Some(address.to_string());

                let count = self.fetch_messages_count().await?;
                self.messages_count = count as usize;
                self.user_name = Some(user_name.to_string());
                Ok(ConnectionResult {
                    success: true,
                    message: format!("Connected to {}", address),
                })
            }
            Err(e) => Err(anyhow::anyhow!("Failed to connect: {}", e)),
        }
    }

    pub async fn disconnect(&mut self) {
        self.address = None;
        self.messages_count = 0;
        self.user_name = None;
    }

    pub async fn send_message(&self, message: &str) -> Result<()> {
        if let Some(address) = &self.address {
            let mut conn = TcpStream::connect(address).await?;
            let formatted_message =
                format!("\u{25B2}<{}> {}", self.user_name.clone().unwrap(), message);
            conn.write_all(format!("\x01{}", formatted_message).as_bytes()).await?;
            println!("Message sent: {}", formatted_message);
            Ok(())
        } else {
            Err(anyhow::anyhow!("No server address set"))
        }
    }

    pub async fn fetch_messages_count(&self) -> Result<u64> {
        if let Ok(mut conn) = TcpStream::connect(self.address.clone().unwrap()).await {
            conn.write_all(&[0x00]).await?;
            let mut buffer = [0u8; 1024];
            let n = conn.read(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer[..n]);
            let count = response
                .parse::<u64>()
                .map_err(|_| anyhow::anyhow!("Failed to parse message count from response"))?;
            Ok(count)
        } else {
            Err(anyhow::anyhow!("Failed to connect to server"))
        }
    }

    pub async fn get_messages(&mut self) -> Result<Vec<MessageResponse>> {
        let mut conn = TcpStream::connect(self.address.clone().unwrap()).await?;
        conn.write_all(&[0x00]).await?;
        let mut buffer = [0u8; 1024];
        let n = conn.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        let count = response
            .parse::<usize>()
            .map_err(|_| anyhow::anyhow!("Failed to parse message count from response"))?;

        println!("Current messages count: {}", count);

        if count > self.messages_count {
            println!("Retrieving messages from server");
            conn.write_all(format!("\x02{}", self.messages_count).as_bytes()).await?;
            println!("Requesting messages from server, count: {}", self.messages_count);
            let mut buffer = vec![0u8; count - self.messages_count];
            println!("Buffer size: {}", buffer.len());
            conn.read_exact(&mut buffer).await?;
            println!("Received {} bytes from server", buffer.len());
            let response = String::from_utf8_lossy(&buffer).into_owned();
            println!("Raw response received: {}", response);

            let mut vec_messages = response.split('\n').collect::<Vec<&str>>();
            // Removing last one because it is always empty
            vec_messages.remove(vec_messages.len() - 1);
            println!("Raw messages received: {:?}", vec_messages);
            let messages: Vec<MessageResponse> = format_messages(vec_messages);
            println!("Received messages: {:?}", messages);

            self.messages_count = count;

            Ok(messages)
        } else {
            Ok(vec![])
        }
    }
}
