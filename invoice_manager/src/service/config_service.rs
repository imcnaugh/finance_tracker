use crate::model::NewConfig;
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Configs {
    database: DatabaseConfig,
    company: CompanyConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    path: String,
}

#[derive(Deserialize, Debug)]
pub struct CompanyConfig {
    name: String,
    address: String,
    email: String,
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

pub fn create_config(init_config: NewConfig) -> Result<(), String> {
    let path = get_config_path().unwrap();
    let default_database_path = get_default_database_path().unwrap();

    let content = format!(
        r#"[database]
path = "{}"

[company]
name = "{}"
address = "{}"
email = "{}"
"#,
        default_database_path.to_str().unwrap()
    );

    fs::write(path, content).map_err(|e| format!("Failed to write config file: {}", e))
}

// TODO should really be a result
fn get_config_path() -> Option<PathBuf> {
    #[cfg(feature = "dev")]
    {
        println!("Using dev config");
        return Some(PathBuf::from("config.dev.toml"));
    }

    println!("Using prod config");
    let path = ProjectDirs::from("com", "caffeine-driven-development", "finance-app");
    path.map(|p| p.config_dir().join("config.toml"))
}

// TODO should really be a result
fn get_default_database_path() -> Option<PathBuf> {
    #[cfg(feature = "dev")]
    {
        return Some(PathBuf::from("devdb.sqlite"));
    }

    let path = ProjectDirs::from("com", "caffeine-driven-development", "finance-app");
    path.map(|p| p.config_dir().join("finance_app.sqlite"))
}
