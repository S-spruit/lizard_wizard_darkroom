use uuid::Uuid;
use std::path::PathBuf;
use serde::Serialize;
use crate::appstate::AppState;
use std::sync::Mutex;
use tauri::State;

#[derive(Clone, Serialize)]
pub struct Asset {
    pub id: Uuid,
    pub filename: String,
    pub path: PathBuf,
    pub thumbnail_path: Option<PathBuf>,
    pub ready: bool,
    pub rating: u8
}
impl Asset {
    pub fn new(path: PathBuf, name: String, thumbnail_path: Option<PathBuf>) -> Self {
        Self {
            id: Uuid::new_v4(),
            filename: name,
            path: path,
            thumbnail_path: thumbnail_path,
            ready: false,
            rating: 0
        }
    }
}
#[tauri::command]
pub fn update_asset_ready(id: String, ready: bool, state: State<Mutex<AppState>>) -> Result<(), String> {
     let uuid = Uuid::parse_str(&id)
        .map_err(|e| e.to_string())?;

    let mut app_state = state.lock().unwrap();
    
    if let Some(asset) = app_state.assets.get_mut(&uuid) {
        asset.ready = ready;
        Ok(())
    } else {
        Err("Asset not found".into())
    }
}

#[tauri::command]
pub fn update_asset_rating(id: String, rating: u8, state: State<Mutex<AppState>>) -> Result<(), String> {
     let uuid = Uuid::parse_str(&id)
        .map_err(|e| e.to_string())?;

    let mut app_state = state.lock().unwrap();
    if let Some(asset) = app_state.assets.get_mut(&uuid) {
        asset.rating = rating;
        Ok(())
    } else {
        Err("Asset not found".into())
    }
}