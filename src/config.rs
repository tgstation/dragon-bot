use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    guild_id: u64,
    values: HashMap<String, String>,
}

impl Config {
    pub fn new(guild_id: u64) -> Self {
        Self {
            guild_id,
            values: HashMap::new(),
        }
    }

    pub fn save(&self) {
        let path = std::env::var("CONFIG_PATH").unwrap();
        let path = std::path::PathBuf::from(path);
        let path = path.join(format!("{}.json", self.guild_id));
        let data = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(&path, data).unwrap();
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        self.save();
    }
}
