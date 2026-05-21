use crate::mediapool::asset::Asset;

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