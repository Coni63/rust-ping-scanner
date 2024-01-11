use std::fs::read_to_string;

use serde_yaml::from_str;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub app: AppConfig,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub database_path: String,
    pub whitelist_path: String,
    pub ping_interval: u64,
}


impl Config {
    pub fn load(file_path: &str) -> Config {
        let s = read_to_string(file_path).unwrap();
    
        let config: Config = from_str(&s).unwrap();
    
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load() {
        let config = Config::load("./config/config.yaml");

        assert_eq!(config.app.database_path, "./out/result.sqlite");
        assert_eq!(config.app.whitelist_path, "./out/whitelist.txt");
        assert_eq!(config.app.ping_interval, 60);
    }
}