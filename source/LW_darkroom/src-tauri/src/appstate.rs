use crate::mediapool::asset::Asset;
use serde::Serialize;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct AppState {
    pub assets: HashMap<Uuid, Asset>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new()
        }
    }
}