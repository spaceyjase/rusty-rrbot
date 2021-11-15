extern crate orca;
use orca::App;
use serde::Deserialize;
use serde_json::Result;
use std::fs;
use fancy_regex::Regex;

#[macro_use]
extern crate lazy_static;

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

lazy_static! {
  static ref RE: Regex = {
    let re = Regex::new(r#"^(?i)rr\?$|^(w[h]?at|wtf)('s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?[\?\.]?$|(?<!")(?<! is |did )(w[h]?at|wtf)('s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?(?!outine| \w)[\?\.]?(?!")|define rr[\?\.]?"#).unwrap();
    re
  };
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

  #[test]
  fn test_regex_match_rr() {
    let query = "rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_is_the_rr() {
    let query = "what is the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }
  
  #[test]
  fn test_regex_match_what_the_rr() {
    let query = "what the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }
  
  #[test]
  fn test_regex_match_what_is_rr() {
    let query = "what is rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_rr_qm() {
    let query = "what rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_rr() {
    let query = "what rr".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_s_the_rr() {
    let query = "what's the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_s_the_rr_quoted() {
    let query = r#""what's the rr?""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_s_the_rr_quoted_text() {
    let query = r#"Sarcasm, "What's the rr?" folks snafu."#.to_string();
    assert!(!RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_s_the_rr_quoted_text_case() {
    let query = r#""What's the RR?""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_s_the_rr_quoted_text_no_match() {
    let query = r#"When somebody asks, "What is the RR?".""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_is_the_rr_and_match() {
    let query = "I'm new to this sub and I would like to ask for some help: What is the RR and where do I find it? ^".to_string();
    assert!(RE.is_match(&query).unwrap());
  }
  
  #[test]
  fn test_regex_match_what_is_rr_with_text() {
    let query = "can someone tell me what is rr? have i missed something?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_is_the_rr_with_text() {
    let query = "can someone tell me what is the rr? have i missed something?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }
  
  #[test]
  fn test_regex_match_also_what_is_the_rr() {
    let query = "Also, what is rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }
  
  
  #[test]
  fn test_regex_match_what_is_rr_match() {
    let query = "what is rr".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_wat_rr_match() {
    let query = "wat rr".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_what_is_rr_match_2() {
    let query = "what is rr.".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_rr_match() {
    let query = "RR?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_quote_rr_quote_no_match() {
    let query = r#""rr?""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_text_what_the_rr_no_match() {
    let query = "I tried to decipher what the RR was asking me to do in a workout...".to_string();
    assert!(!RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_text_what_does_the_rr_match() {
    let query = "what does the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_text_what_is_the_rr_stand_for_match() {
    let query = "what is the rr stand for?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_text_what_does_the_rr_stand_for_match() {
    let query = "what does the rr stand for?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }
  
  #[test]
  fn test_regex_match_text_what_does_the_rr_mean_match() {
    let query = "what does rr mean?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }
  
  #[test]
  fn test_regex_match_text_foo_define_rr_bar_match() {
    let query = "foo define rr bar".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_text_define_rr_match() {
    let query = "define rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_text_define_rr_spacing_match() {
    let query = "define rr    ggsddg".to_string();
    assert!(RE.is_match(&query).unwrap());
  }

  #[test]
  fn test_regex_match_text_no_match() {
    let query = "define nothing rr    ggsddg".to_string();
    assert!(!RE.is_match(&query).unwrap());
  }
}