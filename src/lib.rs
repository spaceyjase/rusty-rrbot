extern crate orca;
use orca::data::Comment;
use orca::data::Listing;
use orca::{App, Sort};
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
  pub hot_take: u8,
}

impl Config {
  pub fn new(config: &str) -> Result<Config> {
    let config: Config = serde_json::from_str(&config).expect("Error parsing config file");

    Ok(config)
  }
}

#[derive(Deserialize)]
pub struct Post {
  pub id: String,
  pub title: String,
  pub selftext: String,
}

impl Post {
  pub fn new(json: &str) -> Result<Post> {
    let post: Post = serde_json::from_str(&json).expect("Error parsing json");

    Ok(post)
  }
  pub fn comments(&self, reddit: &App) -> Listing<Comment> {
    let comments = reddit.get_comment_tree(&self.id).unwrap_or_default();
    comments
  }
}

lazy_static! {
  static ref RE: Regex = {
    let re = Regex::new(r#"^(?i)rr\?$|^(w[h]?at|wtf)([`']s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?[\?\.]?$|(?<!")(?<! is |did )(w[h]?at|wtf)('s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?(?!outine| \w)[\?\.]?(?!")|define rr[\?\.]?"#).unwrap();
    re
  };
  static ref REPLY: String = {
    "The RR is the [Recommended Routine](https://www.reddit.com/r/bodyweightfitness/wiki/kb/recommended_routine).\n*****\n^(I am a bot, flex-beep-boop)".to_string()
  };
  static ref GOOD_BOT: String = {
    "good bot".to_string()
  };
}

fn init_app() -> (App, Config) {
  let contents = fs::read_to_string("config.json").expect("Error reading config file");
  let config = Config::new(&contents).unwrap();
  let mut reddit = App::new("Linux:com.jasonmichaeladams.rrbot", "0.2", "u/spaceyjase").unwrap();
  reddit.authorize_script(&config.client_id, &config.client_secret, &config.username, &config.password).unwrap();
  (reddit, config)
}

pub fn run() -> Result<()> {
  let (reddit, config) = init_app();
  let posts = reddit.get_posts("bodyweightfitness", Sort::Hot).unwrap();

  let mut count = 0;
  for json in posts["data"]["children"].as_array().unwrap() {
    if count > config.hot_take { break };
    let post = Post::new(&json["data"].to_string()).unwrap();
    println!("{}:{}", post.id, post.title);
    for comment in post.comments(&reddit) {
      println!("\t- {}", comment.body);
    }
    count += 1;
  }

  Ok(())
}

#[cfg(test)]
mod test;