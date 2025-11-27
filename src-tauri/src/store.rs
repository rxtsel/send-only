use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "settings.json";
const API_KEY_RECORD: &str = "resend_api_key";
const FROM_EMAILS_KEY: &str = "from_emails";
const PROFILE_KEY: &str = "profile";

// Api Key Management
#[tauri::command]
pub fn has_api_key(app: AppHandle<Wry>) -> Result<bool, String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    let has_key = store.get(API_KEY_RECORD).is_some();

    Ok(has_key)
}

#[tauri::command]
pub fn save_api_key(app: AppHandle<Wry>, api_key: String) -> Result<(), String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    store.set(API_KEY_RECORD, json!(api_key));

    store
        .save()
        .map_err(|e| format!("[ERROR] Failed to save store: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_api_key(app: AppHandle<Wry>) -> Result<Option<String>, String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    match store.get(API_KEY_RECORD) {
        Some(value) => {
            let api_key = value
                .as_str()
                .ok_or_else(|| "[ERROR] API key is not a string".to_string())?
                .to_string();

            println!("[INFO] API key retrieved successfully");
            Ok(Some(api_key))
        }
        None => {
            println!("[INFO]  API key not found");
            Ok(None)
        }
    }
}

#[tauri::command]
pub fn delete_api_key(app: AppHandle<Wry>) -> Result<(), String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("ERROR Failed to load store: {}", e))?;

    let deleted = store.delete(API_KEY_RECORD);

    if !deleted {
        println!("[WARN] API key was not found in store");
    }

    store
        .save()
        .map_err(|e| format!("ERROR Failed to save store: {}", e))?;

    println!("[INFO] API key deleted successfully");

    Ok(())
}

// From emails management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromEmail {
    pub id: String,
    pub label: String,
    pub address: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

pub(crate) fn load_from_emails(app: &AppHandle<Wry>) -> Result<Vec<FromEmail>, String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    let value = store.get(FROM_EMAILS_KEY);

    if let Some(raw) = value {
        let emails: Vec<FromEmail> = serde_json::from_value(raw.clone())
            .map_err(|e| format!("[ERROR] Failed to parse from_emails: {}", e))?;
        Ok(emails)
    } else {
        Ok(Vec::new())
    }
}

pub(crate) fn save_from_emails(app: &AppHandle<Wry>, emails: &[FromEmail]) -> Result<(), String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    store.set(FROM_EMAILS_KEY, json!(emails));

    store
        .save()
        .map_err(|e| format!("[ERROR] Failed to save store: {}", e))?;

    Ok(())
}

// Profile Management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: String,
    pub username: String,
    pub domain: String,
}

pub(crate) fn load_profile(app: &AppHandle<Wry>) -> Result<Option<Profile>, String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    let value = store.get(PROFILE_KEY);

    if let Some(raw) = value {
        let profile: Profile = serde_json::from_value(raw.clone())
            .map_err(|e| format!("[ERROR] Failed to parse profile: {}", e))?;
        Ok(Some(profile))
    } else {
        Ok(None)
    }
}

pub(crate) fn save_profile(app: &AppHandle<Wry>, profile: &Profile) -> Result<(), String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("[ERROR] Failed to load store: {}", e))?;

    store.set(PROFILE_KEY, json!(profile));

    store
        .save()
        .map_err(|e| format!("[ERROR] Failed to save store: {}", e))?;

    Ok(())
}
