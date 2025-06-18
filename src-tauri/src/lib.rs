use crate::message::MessageResponse;
use client::Client;
use tauri::{Manager, State};
use tokio::sync::Mutex;

mod client;
mod clients;
mod formatter;
mod message;
#[tauri::command]
async fn setup_connection(
    state: State<'_, Mutex<Client>>,
    address: String,
    user_name: String,
) -> Result<(), String> {
    let mut client = state.lock().await;

    if let Err(e) = client.setup_connection(&address, &user_name).await {
        return Err(format!("Failed to set up connection: {}", e));
    }

    Ok(())
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
async fn send_message(state: State<'_, Mutex<Client>>, message: String) -> Result<(), String> {
    let client = state.lock().await;
    if let Err(e) = client.send_message(&message).await {
        return Err(format!("Failed to send message: {}", e));
    }
    Ok(())
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
        .invoke_handler(tauri::generate_handler![
            setup_connection,
            disconnect,
            send_message,
            fetch_messages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
