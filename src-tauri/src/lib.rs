use crate::formatter::format_messages;
use crate::message::MessageResponse;
use rac::async_client::Client;
use rac::async_wrac::WClient;
use rac::shared::Credentials;
use tauri::{Manager, State};
use tokio::sync::Mutex;

mod clients;
mod formatter;
mod message;

#[derive(Default, Debug)]
enum ProtocolClient {
    Rac(Client),
    Wrac(WClient),
    #[default]
    Nothing,
}

#[tauri::command]
async fn setup_connection(
    state: State<'_, Mutex<ProtocolClient>>,
    address: String,
    user_name: String,
    password: String,
    use_tls: bool,
) -> Result<(), String> {
    let mut client = state.lock().await;

    if address.starts_with("rac://") {
        let normal_addr = address.replace("rac://", "");
        let mut rac_client = Client::new(
            &normal_addr,
            Credentials {
                username: user_name,
                password: if password.is_empty() {
                    None
                } else {
                    Some(password.clone())
                },
            },
            use_tls,
        );
        rac_client
            .fetch_messages_size()
            .await
            .map_err(|e| e.to_string())?;
        if !password.is_empty() {
            rac_client
                .register_user()
                .await
                .map_err(|e| e.to_string())?;
        }
        *client = ProtocolClient::Rac(rac_client);
    } else if address.starts_with("wrac://") {
        let normal_address = address.replace("wrac://", "");
        let mut new_client = WClient::new(
            &normal_address,
            Credentials {
                username: user_name,
                password: if password.is_empty() {
                    None
                } else {
                    Some(password.clone())
                },
            },
            use_tls,
        );
        new_client.prepare().await.map_err(|e| e.to_string())?;
        new_client
            .fetch_messages_size()
            .await
            .map_err(|e| e.to_string())?;
        if !password.is_empty() {
            new_client
                .register_user()
                .await
                .map_err(|e| e.to_string())?;
        }
        *client = ProtocolClient::Wrac(new_client);
    } else {
        return Err("Invalid connection string.".to_string());
    }
    Ok(())
}

#[tauri::command]
async fn disconnect(state: State<'_, Mutex<ProtocolClient>>) -> Result<(), String> {
    let mut client = state.lock().await;
    *client = ProtocolClient::Nothing;
    Ok(())
}

#[tauri::command]
async fn fetch_messages(
    state: State<'_, Mutex<ProtocolClient>>,
) -> Result<Vec<MessageResponse>, String> {
    let mut client = state.lock().await;
    match &mut *client {
        ProtocolClient::Rac(rac) => {
            let messages = rac
                .fetch_new_messages()
                .await
                .map_err(|e| e.to_string())?
                .to_vec();
            let formatted = format_messages(messages);
            Ok(formatted)
        }
        ProtocolClient::Wrac(wrac) => {
            let messages = wrac
                .fetch_new_messages()
                .await
                .map_err(|e| e.to_string())?
                .to_vec();
            let formatted = format_messages(messages);
            Ok(formatted)
        }
        ProtocolClient::Nothing => Err("Client is not initialized".to_string()),
    }
}

#[tauri::command]
async fn send_message(
    state: State<'_, Mutex<ProtocolClient>>,
    message: String,
) -> Result<(), String> {
    let mut client = state.lock().await;
    match &mut *client {
        ProtocolClient::Rac(rac) => {
            rac.send_custom_message(format!("\u{25B2}<{}> {}", rac.username(), message).as_str())
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        ProtocolClient::Wrac(wrac) => {
            wrac.send_custom_message(format!("\u{25B2}<{}> {}", wrac.username(), message).as_str())
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        ProtocolClient::Nothing => Err("Client is not initialized".to_string()),
    }
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize an AppState with no Client by default.
            app.manage(Mutex::new(ProtocolClient::Nothing));

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
