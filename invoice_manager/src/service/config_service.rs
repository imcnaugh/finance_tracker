use crate::model::{
    CompanyConfiguration, Configuration, DatabaseConfiguration, NewCompanyConfiguration,
};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn get_config() -> Result<Configuration, String> {
    let path = get_config_path().unwrap();

    println!("Config path: {:?}", path);

    if !path.exists() {
        println!("Config file does not exist at path: {:?}", path);
        return Err("Config file does not exist".to_string());
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))?
}

pub fn create_config(init_config: NewCompanyConfiguration) -> Result<(), String> {
    let path = get_config_path().unwrap();
    let default_database_path = get_default_database_path().unwrap();
    let default_database_path = default_database_path.to_str().unwrap();

    let db_config = DatabaseConfiguration::new(default_database_path, None);
    let company_config = CompanyConfiguration::new(
        init_config.get_company_name(),
        init_config.get_company_address(),
        init_config.get_company_email(),
    );

    let configs = Configuration::new(Some(db_config), Some(company_config));

    let configs_as_str = toml::to_string(&configs).unwrap();

    fs::write(path, configs_as_str).map_err(|e| format!("Failed to write config file: {}", e))
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
