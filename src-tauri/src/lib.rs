use results::ConnectionResult;
use server::Server;
use tokio::sync::Mutex; // Change to tokio's Mutex
use tauri::{Manager, State};

mod clients;
mod formatter;
mod message;
mod results;
mod server;

#[tauri::command]
async fn setup_connection(state: State<'_, Mutex<Server>>, address: String) -> Result<ConnectionResult, String> {
    let mut server = state.lock().await;
    
    match server.setup_connection(&address).await {
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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {

            app.manage(Mutex::new(Server::new()));
            if cfg!(debug_assertions) {

                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![setup_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
