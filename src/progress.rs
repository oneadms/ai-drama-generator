use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub completed: Vec<u32>,
    pub last_context: String,
}

impl Progress {
    pub fn load(path: &str) -> Result<Self> {
        match fs::read_to_string(path) {
            Ok(data) => Ok(serde_json::from_str(&data)?),
            Err(_) => Ok(Self { completed: vec![], last_context: String::new() }),
        }
    }
    
    pub fn save(&self, path: &str) -> Result<()> {
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
