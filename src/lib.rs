use std::collections::HashSet;
use crate::reddit::RedditApp;
use crate::reddit::Reddit;
use crate::post::Post;
use failure::Error;
use std::cmp;
use std::fs;

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

  // get existing replied to posts
  let mut comments_db = fs::read_to_string(&app.config.comments_db_filename)
    .expect("Missing comments db file")   // TODO: file can be created if it doesn't exist
    .lines()
    .map(|x| x.to_string())
    .collect::<HashSet<String>>();
  let mut posts_db = fs::read_to_string(&app.config.posts_db_filename)
    .expect("Missing posts db file")
    .lines()
    .map(|x| x.to_string())
    .collect::<HashSet<String>>();

  // get new posts and check for post and comment matches
  let posts = app.get_posts();
  let count = cmp::min(posts.len(), app.config.hot_take as usize);
  for json in &posts[0..count] {
    let post = Post::new(&json["data"].to_string(), &app).unwrap();
    if post.is_match().unwrap() {
      if !posts_db.contains(&post.id) {
        println!("Replying to post {}", post.id);
        //reddit.reply(&id, &REPLY).unwrap();
        posts_db.insert(post.id.to_string());
      }
    }
    post.get_matching_comments()
        .filter(|id| !comments_db.contains(id)).collect::<Vec<_>>()
        .iter_mut()
        .for_each(|id| {
          println!("Replying to comment {}", id);
          //reddit.reply(&id, &REPLY).unwrap();
          comments_db.insert(id.to_string());
        });
  }

  Ok(())
}

#[cfg(test)]
mod test;