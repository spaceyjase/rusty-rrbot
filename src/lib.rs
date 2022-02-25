extern crate orca;
use serde_json::Value;
use orca::data::Listing;
use orca::data::Comment;
use crate::config::Config;
use crate::post::Post;
use orca::{App, Sort};
use failure::Error;
use std::fs;
use std::cmp;

mod post;
mod config;

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref REPLY: String = {
    "The RR is the [Recommended Routine](https://www.reddit.com/r/bodyweightfitness/wiki/kb/recommended_routine).\n*****\n^(I am a bot, flex-beep-boop)".to_string()
  };
  static ref GOOD_BOT: String = {
    "good bot".to_string()
  };
}

pub struct Reddit {
  reddit: App,
  config: Config,
}

impl Reddit {
  pub fn new() -> Reddit {
    let contents = fs::read_to_string("config.json").expect("Error reading config file");
    let config = Config::new(&contents).unwrap();
    let mut reddit = App::new("Linux:com.jasonmichaeladams.rrbot", "0.2", "u/spaceyjase").unwrap();
    reddit.authorize_script(&config.client_id, &config.client_secret, &config.username, &config.password).unwrap();
    Reddit{ reddit, config }
  }
}

pub trait RedditApp {
  fn get_posts(&self, sub: &str, sort: Sort) -> Result<Value, Error>;
  fn get_comment_tree(&self, post_id: &str) -> Result<Listing<Comment>, Error>;
}

impl RedditApp for Reddit {
  fn get_comment_tree(self: &Reddit, post_id: &str) -> Result<Listing<Comment>, Error> {
    self.reddit.get_comment_tree(post_id)
  }
  fn get_posts(&self, sub: &str, sort: Sort) -> Result<Value, Error> {
    self.reddit.get_posts(sub, sort)
  }
}

impl Default for Reddit {
  fn default() -> Self { Option::<Self>::None.unwrap() }
}

pub fn run() -> Result<(), Error> {
  let app = Reddit::new();
  let posts = app.get_posts("bodyweightfitness", Sort::Hot).unwrap();

  let posts = posts["data"]["children"].as_array().unwrap();
  let count = cmp::min(posts.len(), app.config.hot_take as usize);
  for json in &posts[0..count] {
    let post = Post::new(&json["data"].to_string(), &app).unwrap();
    if post.is_match().unwrap() {
      println!("Matched! {}", post.id);
    }
    post.get_matching_comments().unwrap().iter().for_each(|id| {
      //reddit.reply(&id, &REPLY).unwrap();
      println!("Matched! {}", id);
    });
  }

  Ok(())
}

#[cfg(test)]
mod test;