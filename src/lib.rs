extern crate orca;
use crate::config::Config;
use crate::post::Post;
use orca::{App, Sort};
use serde_json::Result;
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

  let posts = posts["data"]["children"].as_array().unwrap();
  let count = cmp::min(posts.len(), config.hot_take as usize);
  for json in &posts[0..count] {
    let post = Post::new(&json["data"].to_string()).unwrap();
    if post.is_match().unwrap() {
      println!("Matched! {}", post.id);
    }
    post.get_matching_comments(post.comments(&reddit)).unwrap().iter().for_each(|id| {
      //reddit.reply(&id, &REPLY).unwrap();
      println!("Matched! {}", id);
    });
  }

  Ok(())
}

#[cfg(test)]
mod test;