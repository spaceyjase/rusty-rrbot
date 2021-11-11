extern crate orca;
use orca::App;
use serde::Deserialize;
use serde_json::Result;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
  pub client_id: String,
  pub client_secret: String,
  pub username: String,
  pub password: String,
}

impl Config {
  pub fn new(config: &str) -> Result<Config> {
    let config: Config = serde_json::from_str(&config).expect("Error parsing config file");

    Ok(config)
  }
}

pub fn run() -> Result<()> {
  let contents = fs::read_to_string("config.json").expect("Error reading config file");

  let config = Config::new(&contents).unwrap();

  let mut reddit = App::new("Linux:com.jasonmichaeladams.rrbot", "0.1", "u/spaceyjase").unwrap();
  reddit.authorize_script(&config.client_id, &config.client_secret, &config.username, &config.password).unwrap();

  let user =  reddit.get_self().unwrap();
  println!("Got data: {}", user);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_config_parse() {
    let config = r#"
      {
        "client_id": "client_id",
        "client_secret": "client_secret",
        "username": "username",
        "password": "password"
      }
    "#;

    let config = Config::new(&config).unwrap();

    assert_eq!(config.client_id, "client_id");
    assert_eq!(config.client_secret, "client_secret");
    assert_eq!(config.username, "username");
    assert_eq!(config.password, "password");
  }
}