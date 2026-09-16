use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct App {
    pub source_folder: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            source_folder: String::new(),
        }
    }

    fn resolve_path(filename: &str) -> std::path::PathBuf {
        dirs::home_dir().unwrap().join(filename)
    }

    pub fn save(&self, filename: &str) {
        let path = Self::resolve_path(filename);
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json).unwrap();
    }

    pub fn load(filename: &str) -> Self {
        let path = Self::resolve_path(filename);
        let json = fs::read_to_string(path).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}
