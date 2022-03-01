use crate::reddit::RedditApp;
use fancy_regex::Regex;
use orca::data::Comment;
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
    Regex::new(r#"^(?i)rr\?$|^(w[h]?at|wtf)([`']s|\u{2019}s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?[\?\.]?$|(?<!")(?<! is |did )(w[h]?at|wtf)('s|\u{2019}s| is| does)? (a[n]? rr|the rr|rr)( mean| stand for| and where do i find it)?(?!outine| \w)[\?\.]?(?!")|define rr[\?\.]?"#).unwrap()
  };
}

fn is_text_match(text: &str) -> Result<bool> {
  if RE.is_match(&text).unwrap_or(false) {
    return Ok(true)
  }
  Ok(false)
}

impl<'a, T> Post<'a, T>
where T: RedditApp + std::default::Default
{
  pub fn new(json: &str, reddit: &'a T) -> Result<Post<'a, T>> {
    let mut post: Post<T> = serde_json::from_str(&json).expect("Error parsing json");
    post.reddit = Option::from(reddit);

    Ok(post)
  }
  pub fn comments(&self) -> impl Iterator<Item=Comment> {
    self.reddit.unwrap().get_comment_tree(&self.id).unwrap_or_default()
  }
  pub fn is_match(&self) -> Result<bool> {
    is_text_match(&self.selftext)
  }
  pub fn get_matching_comments(&self) -> impl Iterator<Item=String> {
    self.comments()
      .into_iter()
      .filter(|comment| is_text_match(&comment.body).unwrap_or(false))
      .map(|comment| comment.id)
  }
}