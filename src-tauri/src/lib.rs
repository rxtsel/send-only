mod config;
mod email;
mod profile;
mod store;
use tauri_plugin_store::StoreExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize the store
            let _store = app.store("settings.json")?;
            println!("🚀 Store initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            store::has_api_key,
            store::save_api_key,
            store::get_api_key,
            store::delete_api_key,
            email::send_email,
            config::list_from_emails,
            config::create_from_email,
            config::update_from_email,
            config::delete_from_email,
            profile::get_profile,
            profile::save_profile_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
