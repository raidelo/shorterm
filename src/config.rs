use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub key: String,
    #[serde(default)]
    pub modifier: Vec<String>,
}

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .expect("no se pudo determinar el home directory")
        .join(".config")
        .join("shorterm")
        .join("config.toml")
}

fn default_config_toml() -> &'static str {
    r#"key = "F9"
modifier = []
"#
}

pub fn load_config() -> Config {
    let path = config_path();

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("no se pudo crear .config/shorterm");
        }
        fs::write(&path, default_config_toml()).expect("no se pudo escribir config.toml");
    }

    let contents = fs::read_to_string(&path).expect("no se pudo leer config.toml");
    toml::from_str(&contents).expect("config.toml mal formado")
}
