use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub lambda: usize,
    pub t:usize
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("config/config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::de::from_str(&contents)?;
    Ok(config)
}