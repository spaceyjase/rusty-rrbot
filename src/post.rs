use crate::RedditApp;
use fancy_regex::Regex;
use orca::data::Comment;
use orca::data::Listing;
use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize)]
pub struct Post<'a, T: RedditApp>
where T: RedditApp,
{
  pub id: String,
  pub title: String,
  pub selftext: String,
  #[serde(skip_deserializing)]
  reddit: Option<&'a T>,
}

lazy_static! {
  pub static ref RE: Regex = {
    Regex::new(r#"^(?i)rr\?$|^(w[h]?at|wtf)([`']s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?[\?\.]?$|(?<!")(?<! is |did )(w[h]?at|wtf)('s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?(?!outine| \w)[\?\.]?(?!")|define rr[\?\.]?"#).unwrap()
  };
}

impl<'a, T> Post<'a, T>
where T: RedditApp + std::default::Default
{
  pub fn new(json: &str, reddit: &'a T) -> Result<Post<'a, T>> {
    let mut post: Post<T> = serde_json::from_str(&json).expect("Error parsing json");
    post.reddit = Option::from(reddit);

    Ok(post)
  }
  pub fn comments(&self) -> Listing<Comment> {
    self.reddit.unwrap().get_comment_tree(&self.id).unwrap_or_default()
  }
  pub fn is_match(&self) -> Result<bool> {
    self.is_text_match(&self.selftext)
  }
  pub fn get_matching_comments(&self) -> Result<Vec<String>> {
    let mut results = Vec::new();
    self.comments()
      .filter(|comment| self.is_text_match(&comment.body).unwrap_or(false))
      .for_each(|comment| results.push(comment.id));

    Ok(results)
  }
  fn is_text_match(&self, text: &str) -> Result<bool> {
    if RE.is_match(&text).unwrap_or(false) {
      return Ok(true)
    }
    Ok(false)
  }
}