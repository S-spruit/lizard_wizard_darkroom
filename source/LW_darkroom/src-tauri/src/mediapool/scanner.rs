use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io::Result;
use crate::mediapool::asset::Asset;

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
pub fn scan_and_build(path: String) -> Vec<Asset> {
    let paths = scan_folder(path);
    build_assets(paths)
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

use uuid::Uuid;

fn build_assets(paths: Vec<PathBuf>) -> Vec<Asset> {
    paths
        .into_iter()
        .map(|path| crate::mediapool::asset::Asset {
            id: Uuid::new_v4(),
            filename: path.clone().file_name().and_then(|name| name.to_str()).unwrap_or("unknown").to_string(),
            path: path.clone(),
            thumbnail_path: Some(path.clone())
        })
        .collect()
}