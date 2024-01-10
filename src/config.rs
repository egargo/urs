use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct UrsConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct CorsConfig {
    pub server: String,
    pub client: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub urs: UrsConfig,
    pub redis: RedisConfig,
    pub cors: CorsConfig,
}

impl Config {
    fn file(filename: &str) -> Result<Self> {
        Ok(from_str(&read_to_string(filename)?)?)
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::file("urs.toml").unwrap();
}
