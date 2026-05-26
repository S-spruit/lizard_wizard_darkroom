use std::fs;
use std::path::Path;
use std::path::PathBuf;
use crate::mediapool::asset::Asset;
use crate::appstate::AppState;
use crate::rawengine::rawdecoder::{extract_thumbnails, get_cache_path};
use std::sync::Mutex;
use tauri::State;
use std::collections::HashMap;
use uuid::Uuid;

fn is_raw(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "cr2" | "cr3" | "nef" | "arw" | "raf" | "dng" | "rw2" | "jpg"
        ),
        None => false,
    }
}

#[tauri::command]
pub fn scan_and_build(path: String, state: State<Mutex<AppState>>) {
    let mut app_state = state.lock().unwrap();
    let paths = scan_folder(path);
    let assets = build_assets(paths);
    app_state.assets.clear();
    for asset in assets {
    app_state.assets.insert(asset.id, asset);

    }
}

#[tauri::command]
pub fn get_assets(state: State<Mutex<AppState>>) -> HashMap<Uuid, Asset> {
    let app_state = state.lock().unwrap();

    app_state.assets.clone()
}



fn scan_folder(path: String) -> Vec<PathBuf> {
    let mut results = Vec::new();
    walk(std::path::Path::new(&path), &mut results);
    results
}

pub fn walk(dir: &Path, results: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            walk(&path, results);
        } else if is_raw(&path) {
            results.push(path);
        }
    }
}


fn build_assets(paths: Vec<PathBuf>) -> Vec<Asset> {
    paths
        .into_iter()
        .map(|path|{
            let cache_path = get_cache_path(&path);

            let thumbnail_path = extract_thumbnails(&path, &cache_path);
            crate::mediapool::asset::Asset {
            id: Uuid::new_v4(),
            filename: path.clone().file_name().and_then(|name| name.to_str()).unwrap_or("unknown").to_string(),
            path: path.clone(),
            thumbnail_path: thumbnail_path,
            rating: 0,
            ready: false
        
        }
        })
        .collect()
}