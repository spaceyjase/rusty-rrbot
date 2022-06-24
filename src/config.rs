use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize)]
pub struct Config {
  pub client_id: String,
  pub client_secret: String,
  pub username: String,
  pub password: String,
  pub hot_take: u8,
  pub inbox_db_filename: String,
  pub posts_db_filename: String,
  pub comments_db_filename: String,
  pub sub: String,
  pub dry_run: bool,
}

impl Config {
  pub fn new(config: &str) -> Result<Config> {
    let config: Config = serde_json::from_str(&config).expect("Error parsing config file");

    Ok(config)
  }
}