use tauri::{AppHandle, Wry};

use crate::store::{load_profile, save_profile, Profile};

#[tauri::command]
pub fn get_profile(app: AppHandle<Wry>) -> Result<Option<Profile>, String> {
    load_profile(&app)
}

#[tauri::command]
pub fn save_profile_command(
    app: AppHandle<Wry>,
    first_name: String,
    last_name: String,
    username: String,
    domain: String,
) -> Result<(), String> {
    let profile = Profile {
        first_name,
        last_name,
        username,
        domain,
    };
    save_profile(&app, &profile)
}
