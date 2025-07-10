use crate::formatter::format_messages;
use crate::message::MessageResponse;
use rac::async_client::Client;
use rac::async_wrac::WClient;
use rac::shared::{ClientError, Credentials};
use std::ops::Deref;
use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;

mod clients;
mod formatter;
mod message;

#[derive(Default, Debug)]
pub enum ProtocolClient {
    Rac(Box<Client>),
    Wrac(Box<WClient>),
    #[default]
    Nothing,
}

#[derive(Default, Debug)]
pub struct AppState {
    pub client: Arc<Mutex<ProtocolClient>>,
    pub avatar: Arc<Mutex<Option<String>>>,
}

#[tauri::command]
async fn setup_connection(
    app_state: State<'_, AppState>,
    address: String,
    user_name: String,
    password: String,
    use_tls: bool,
    avatar: String,
) -> Result<(), String> {
    let mut client = app_state.client.lock().await;
    let mut avatar_lock = app_state.avatar.lock().await;

    if !avatar.is_empty() {
        *avatar_lock = Some(avatar);
    }

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
            let response = rac_client.register_user().await;
            if let Err(e) = response {
                match e {
                    ClientError::UsernameAlreadyTaken => {}
                    _ => return Err(e.to_string()),
                }
            }
        }
        *client = ProtocolClient::Rac(Box::new(rac_client));
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
            let response = new_client.register_user().await;
            if let Err(e) = response {
                match e {
                    ClientError::UsernameAlreadyTaken => {}
                    _ => return Err(e.to_string()),
                }
            }
        }
        *client = ProtocolClient::Wrac(Box::new(new_client));
    } else {
        return Err("Invalid connection string.".to_string());
    }
    Ok(())
}

#[tauri::command]
async fn disconnect(app_state: State<'_, AppState>) -> Result<(), String> {
    let mut client = app_state.client.lock().await;
    *client = ProtocolClient::Nothing;
    Ok(())
}

#[tauri::command]
async fn fetch_messages(app_state: State<'_, AppState>) -> Result<Vec<MessageResponse>, String> {
    let mut client = app_state.client.lock().await;
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
async fn send_message(app_state: State<'_, AppState>, message: String) -> Result<(), String> {
    let mut client = app_state.client.lock().await;
    let avatar = app_state.avatar.lock().await;

    match &mut *client {
        ProtocolClient::Rac(rac) => {
            println!(
                "{:?}",
                format!(
                    "\u{25B2}<{}> {}{}",
                    rac.username(),
                    message,
                    if let Some(url) = avatar.deref() {
                        format!("\x06!!AR!!{url}")
                    } else {
                        String::new()
                    }
                )
            );
            rac.send_custom_message(
                format!(
                    "\u{25B2}<{}> {}{}",
                    rac.username(),
                    message,
                    if let Some(url) = avatar.deref() {
                        format!("\x06!!AR!!{url}")
                    } else {
                        String::new()
                    }
                )
                .as_str(),
            )
            .await
            .map_err(|e| e.to_string())?;
            Ok(())
        }
        ProtocolClient::Wrac(wrac) => {
            wrac.send_custom_message(
                format!(
                    "\u{25B2}<{}> {}{}",
                    wrac.username(),
                    message,
                    if let Some(url) = avatar.deref() {
                        format!("\x06!!AR!!{url}")
                    } else {
                        String::new()
                    }
                )
                .as_str(),
            )
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
            app.manage(AppState::default());

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
