mod crypto;
mod database;

use std::sync::Mutex;
use tauri::{Manager, State};

// State management for the database connection
pub struct DbState(pub Mutex<sled::Db>);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn encrypt(data: String, password: String) -> Result<String, String> {
    crypto::encrypt(&data, &password)
}

#[tauri::command]
fn decrypt(encoded_data: String, password: String) -> Result<String, String> {
    crypto::decrypt(&encoded_data, &password)
}

#[tauri::command]
fn add_item(name: String, content: String, state: State<DbState>) -> Result<(), String> {
    let encrypted_content = crypto::encrypt(&content, "db-test-password")?;

    let db = state.0.lock().unwrap();
    db.insert(name.as_bytes(), encrypted_content.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let app_data_dir = handle.path().app_data_dir().expect("Failed to get app data dir");
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
            }
            let db_path = app_data_dir.join("vault.db");

            let db = sled::open(&db_path).expect("Failed to open database");

            // Add the connection to Tauri's managed state
            app.manage(DbState(Mutex::new(db)));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, encrypt, decrypt, add_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
