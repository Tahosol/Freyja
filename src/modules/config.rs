use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub whisper_model_path: String,
    pub ollama_host: String,
    pub ollama_port: u16,
    pub ollama_model: String,
}

use std::fs;
pub fn get() -> Config {
    let path = dirs::config_dir()
        .expect("Fail to find config/freyja path")
        .join("freyja")
        .join("config.toml");
    println!("{path:?}");
    let string = fs::read_to_string(path).expect("Fail to read config.toml");
    let config: Config = toml::from_str(&string).expect("Fail to deserialize config.toml");
    config
}
