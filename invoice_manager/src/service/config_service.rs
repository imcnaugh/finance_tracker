use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Configs {
    db_path: String,
}

fn get_config_path() -> Option<PathBuf> {
    #[cfg(feature = "dev")]
    {
        println!("Using dev config");
        return Some(PathBuf::from("config.toml"));
    }

    println!("Using prod config");
    let path = ProjectDirs::from("com", "caffeine-driven-development", "finance-app");
    path.map(|p| p.config_dir().join("config.toml"))
}

pub fn get_config() -> Result<Configs, String> {
    let path = get_config_path().unwrap();

    println!("Config path: {:?}", path);

    if !path.exists() {
        println!("Config file does not exist at path: {:?}", path);
        return Err("Config file does not exist".to_string());
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))
}
