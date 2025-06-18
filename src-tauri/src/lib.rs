use crate::message::MessageResponse;
use crate::results::SendResult;
use results::ConnectionResult;
use client::Client;
use tauri::{Manager, State};
use tokio::sync::Mutex;
// Change to tokio's Mutex

mod clients;
mod formatter;
mod message;
mod results;
mod client;

#[tauri::command]
async fn setup_connection(
    state: State<'_, Mutex<Client>>,
    address: String,
    user_name: String,
) -> Result<ConnectionResult, String> {
    let mut client = state.lock().await;

    match client.setup_connection(&address, &user_name).await {
        Ok(_) => Ok(ConnectionResult {
            success: true,
            message: "Connection established".to_string(),
        }),
        Err(e) => Ok(ConnectionResult {
            success: false,
            message: format!("Failed to connect: {}", e),
        }),
    }
}

#[tauri::command]
async fn disconnect(state: State<'_, Mutex<Client>>) -> Result<(), String> {
    let mut client = state.lock().await;
    client.disconnect().await;
    Ok(())
}

#[tauri::command]
async fn fetch_messages(state: State<'_, Mutex<Client>>) -> Result<Vec<MessageResponse>, String> {
    let mut client = state.lock().await;
    let messages = client.get_messages().await.map_err(|e| e.to_string())?;
    Ok(messages)
}
#[tauri::command]
async fn send_message(
    state: State<'_, Mutex<Client>>,
    message: String,
) -> Result<SendResult, String> {
    let client = state.lock().await;
    let res = client.send_message(&message).await;
    match res {
        Ok(_) => Ok(SendResult {
            success: true,
            message: "Message sent successfully".to_string(),
        }),
        Err(e) => Ok(SendResult {
            success: false,
            message: format!("Failed to send message: {}", e),
        }),
    }
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Client::new()));
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![setup_connection, disconnect, send_message, fetch_messages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
