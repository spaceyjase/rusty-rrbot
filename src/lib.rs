extern crate orca;
use crate::reddit::RedditApp;
use crate::reddit::Reddit;
use crate::post::Post;
use orca::{Sort};
use failure::Error;
use std::cmp;

mod post;
mod config;
mod reddit;

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