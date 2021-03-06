
use failure::Error;
use orca::data::Comment;
use orca::data::Listing;
use crate::config::Config;
use orca::{App, Sort};
use std::fs;

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
  fn get_posts(&self) -> std::vec::Vec<serde_json::Value>;
  fn reply(&self, id: &str) -> Result<(), Error>;
}

impl RedditApp for Reddit {
  fn get_comment_tree(self: &Reddit, post_id: &str) -> Result<Listing<Comment>, Error> {
    self.reddit.get_comment_tree(post_id)
  }
  fn get_posts(&self) -> std::vec::Vec<serde_json::Value> {
    let posts = self.reddit.get_posts(&self.config.sub, Sort::Hot).unwrap();
    posts["data"]["children"].as_array().unwrap().to_vec()
  }
  fn reply(&self, id: &str) -> Result<(), Error> {
    self.reddit.comment(&REPLY, id)
  }
}

impl Default for Reddit {
  fn default() -> Self { Option::<Self>::None.unwrap() }
}
