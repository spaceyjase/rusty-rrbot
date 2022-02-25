
use failure::Error;
use serde_json::Value;
use orca::data::Comment;
use orca::data::Listing;
use crate::config::Config;
use orca::{App, Sort};
use std::fs;

pub struct Reddit {
  reddit: App,
  pub config: Config,
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
  fn get_comment_tree(&self, post_id: &str) -> Result<Listing<Comment>, Error>;
  fn get_posts(&self) -> Result<Value, Error>;
}

impl RedditApp for Reddit {
  fn get_comment_tree(self: &Reddit, post_id: &str) -> Result<Listing<Comment>, Error> {
    self.reddit.get_comment_tree(post_id)
  }
  fn get_posts(&self) -> Result<Value, Error> {
    self.reddit.get_posts("bodyweightfitness", Sort::Hot)
  }
}

impl Default for Reddit {
  fn default() -> Self { Option::<Self>::None.unwrap() }
}
