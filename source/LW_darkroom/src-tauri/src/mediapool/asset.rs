use uuid::Uuid;
use std::path::PathBuf;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Asset {
    pub id: Uuid,
    pub filename: String,
    pub path: PathBuf,
    pub thumbnail_path: Option<PathBuf>
}
impl Asset {
    pub fn new(path: PathBuf, name: String, thumbnail_path: Option<PathBuf>) -> Self {
        Self {
            id: Uuid::new_v4(),
            filename: name,
            path: path,
            thumbnail_path: thumbnail_path
        }
    }
}