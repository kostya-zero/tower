use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Default)]
pub struct Server {
    connection: Option<TcpStream>,
    messages_count: u64,
}

impl Server {
    pub fn new() -> Self {
        Self {
            connection: None,
            messages_count: 0,
        }
    }

    pub async fn setup_connection(&mut self, address: &str) -> Result<()> {
        if self.connection.is_some() {
            return Err(anyhow::anyhow!("Connection already established"));
        }

        println!("Attempting to connect to {}", address);

        match TcpStream::connect(address).await {
            Ok(stream) => {
                self.connection = Some(stream);
                println!("Connected to {}", address);
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to connect: {}", e)),
        }
    }

    pub async fn get_messages_count(&mut self) -> Result<()> {
        if let Some(ref mut connection) = self.connection {
            // Check if connection is still alive
            if connection.peek(&mut [0]).await.is_err() {
                return Err(anyhow::anyhow!("Connection is no longer alive"));
            }
            
            match connection.write_all(&[0x00]).await {
                Ok(_) => {
                    let mut buffer = [0u8; 1024];
                    match connection.read(&mut buffer).await {
                        Ok(n) => {
                            let response = String::from_utf8_lossy(&buffer[..n]);
                            println!("Received response: {}", response);
                            // Parse the response to get the message count
                            if let Ok(count) = response.parse::<u64>() {
                                self.messages_count = count;
                            } else {
                                return Err(anyhow::anyhow!("Failed to parse message count"));
                            }
                        }
                        Err(e) => return Err(anyhow::anyhow!("Failed to read response: {}", e)),
                    }
                },
                Err(e) => return Err(anyhow::anyhow!("Failed to send request: {}", e)),
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("No connection established"))
        }
    }

}
