use fancy_regex::Regex;
use orca::{App};
use orca::data::Comment;
use orca::data::Listing;
use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize)]
pub struct Post {
  pub id: String,
  pub title: String,
  pub selftext: String,
}

lazy_static! {
  pub static ref RE: Regex = {
    Regex::new(r#"^(?i)rr\?$|^(w[h]?at|wtf)([`']s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?[\?\.]?$|(?<!")(?<! is |did )(w[h]?at|wtf)('s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?(?!outine| \w)[\?\.]?(?!")|define rr[\?\.]?"#).unwrap()
  };
}

impl Post {
  pub fn new(json: &str) -> Result<Post> {
    let post: Post = serde_json::from_str(&json).expect("Error parsing json");

    Ok(post)
  }
  pub fn comments(&self, reddit: &App) -> Listing<Comment> {
    reddit.get_comment_tree(&self.id).unwrap_or_default()
  }
  pub fn is_match(&self) -> Result<bool> {
    self.is_text_match(&self.selftext)
  }
  pub fn get_matching_comments(&self, listing: Listing<Comment>) -> Result<Vec<String>> {
    let mut results = Vec::new();
    for comment in listing {
      if self.is_text_match(&comment.body).unwrap_or(false) {
        results.push(comment.id);
      }
      results.append(&mut self.get_matching_comments(comment.replies).unwrap());
    }
    Ok(results)
  }
  fn is_text_match(&self, text: &str) -> Result<bool> {
    if RE.is_match(&text).unwrap_or(false) {
      return Ok(true)
    }
    Ok(false)
  }
}