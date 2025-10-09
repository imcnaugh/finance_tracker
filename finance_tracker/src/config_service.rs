use crate::configuration::Configuration;
use directories::ProjectDirs;
use invoice_manager::model::{CompanyConfiguration, NewCompanyConfiguration};
use std::fs;
use std::path::PathBuf;
use utilities::database_configuration::DatabaseConfiguration;

pub fn get_config() -> Result<Configuration, String> {
    let path = get_config_path().unwrap();

    if !path.exists() {
        return Err("Config file does not exist".to_string());
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))
}

pub fn create_config(init_config: NewCompanyConfiguration) -> Result<(), String> {
    let path = get_config_path().ok_or("Failed to get config path")?;

    let default_database_path = get_default_database_path()
        .ok_or("Failed to get default database path")?
        .to_str()
        .ok_or("Invalid path")?
        .to_string();
    let db_config = DatabaseConfiguration::new(&default_database_path, None);

    let company_config = CompanyConfiguration::new(
        init_config.get_company_name(),
        init_config.get_company_address(),
        init_config.get_company_email(),
    );

    let configs = Configuration::new(db_config, company_config);

    let configs_as_str = toml::to_string(&configs).unwrap();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?
    }

    fs::write(path, configs_as_str).map_err(|e| format!("Failed to write config file: {}", e))
}

fn get_config_path() -> Option<PathBuf> {
    #[cfg(feature = "dev")]
    {
        return Some(PathBuf::from("config.dev.toml"));
    }

    get_project_dirs().map(|p| p.config_dir().join("config.toml"))
}

fn get_default_database_path() -> Option<PathBuf> {
    #[cfg(feature = "dev")]
    {
        return Some(PathBuf::from("finance_devdb.sqlite"));
    }

    get_project_dirs().map(|p| p.config_dir().join("finance.sqlite"))
}

fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "caffeine-driven-development", "finance-app")
}
