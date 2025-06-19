use crate::formatter::format_messages;
use crate::message::MessageResponse;
use rac::async_client::Client;
use rac::shared::{Connection, Credentials};
use tauri::{Manager, State};
use tokio::sync::Mutex;

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
    client.update_address(address);
    client.update_credentials(Credentials {
        username: user_name.clone(),
        password: None,
    });
    client.test_connection().await.map_err(|e| e.to_string())?;
    client
        .fetch_messages_size()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn disconnect(state: State<'_, Mutex<Client>>) -> Result<(), String> {
    let mut client = state.lock().await;
    client.reset();
    Ok(())
}

#[tauri::command]
async fn fetch_messages(state: State<'_, Mutex<Client>>) -> Result<Vec<MessageResponse>, String> {
    let mut client = state.lock().await;
    let messages = client
        .fetch_new_messages()
        .await
        .map_err(|e| e.to_string())?
        .to_vec();
    let formatted_messages = format_messages(messages);
    Ok(formatted_messages)
}

#[tauri::command]
async fn send_message(state: State<'_, Mutex<Client>>, message: String) -> Result<(), String> {
    let client = state.lock().await;
    client
        .send_custom_message(format!("\u{25B2}<{}> {}", client.username(), message).as_str())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Client::new(
                "".to_string(),
                Credentials::default(),
                Connection::RAC,
            )));
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
