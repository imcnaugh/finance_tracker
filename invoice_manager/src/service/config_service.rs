use directories::ProjectDirs;
use std::path::PathBuf;

fn get_config_path() -> Option<PathBuf> {
    #[cfg(feature = "use-dirs")]
    {
        if let Some(proj) = directories::ProjectDirs::from("com", "josh", "finance") {
            let p = proj.config_dir().join("config.toml");
            if !p.exists() {
                return Some(p);
            }
        }
    }

    let path = ProjectDirs::from("com", "caffeine-driven-development", "finance-app");
    None
}
