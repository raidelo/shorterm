use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub key: String,
    #[serde(default)]
    pub modifier: Vec<String>,
    pub command: String,
    pub args: Vec<String>,
}

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .expect("no se pudo determinar el home directory")
        .join(".config")
        .join("shorterm")
        .join("config.toml")
}

pub fn load_config() -> Config {
    let path = config_path();

    if !path.exists() {
        panic!("el archivo de configuracion en `$HOME/.config/shorterm/config.toml` no existe");
    }

    let contents = fs::read_to_string(&path).expect("no se pudo leer config.toml");
    toml::from_str(&contents).expect("config.toml mal formado")
}
