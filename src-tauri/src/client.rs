use crate::formatter::format_messages;
use crate::message::MessageResponse;
use crate::results::ConnectionResult;
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug, Default)]
pub struct Client {
    pub messages_count: usize,
    user_name: Option<String>,
    address: Option<String>,
}

impl Client {
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
        let addr = self.address.clone().unwrap();
        let mut conn = TcpStream::connect(addr).await?;
        let formatted_message =
            format!("\u{25B2}<{}> {}", self.user_name.clone().unwrap(), message);
        conn.write_all(format!("\x01{}", formatted_message).as_bytes())
            .await?;
        Ok(())
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

        if count > self.messages_count {
            conn.write_all(format!("\x02{}", self.messages_count).as_bytes())
                .await?;
            let mut buffer = vec![0u8; count - self.messages_count];
            conn.read_exact(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer).into_owned();

            let mut vec_messages = response.split('\n').collect::<Vec<&str>>();
            // Removing last one because it is always empty
            vec_messages.remove(vec_messages.len() - 1);
            let messages: Vec<MessageResponse> = format_messages(vec_messages);

            self.messages_count = count;

            Ok(messages)
        } else {
            Ok(vec![])
        }
    }
}
