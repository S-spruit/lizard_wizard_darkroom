use crate::mediapool::asset::Asset;
use serde::Serialize;
#[derive(Clone, Serialize)]
pub struct AppState {
    pub assets: Vec<Asset>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            assets: Vec::new()
        }
    }
}