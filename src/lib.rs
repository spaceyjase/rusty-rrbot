use crate::reddit::post::Post;
use crate::reddit::{Reddit, RedditApp};
use failure::Error;
use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;

mod config;
mod reddit;

#[macro_use]
extern crate lazy_static;

fn get_db(filename: &str) -> HashSet<String> {
    match fs::read_to_string(&filename) {
        Ok(contents) => contents
            .lines()
            .map(|x| x.to_string())
            .collect::<HashSet<_>>(),
        Err(_e) => HashSet::new(),
    }
}

fn write_db(filename: &str, db: &HashSet<String>) {
    let file = match File::create(filename) {
        Err(why) => panic!("couldn't create file: {}", why),
        Ok(file) => file,
    };

    db.iter().for_each(|x| {
        writeln!(&file, "{}", x).expect("couldn't write to file");
    });
}

pub fn run() -> Result<(), Error> {
    let app = Reddit::new();

    // get existing replied to posts, comments and inbox replies
    let mut comments_db = get_db(&app.config().comments_db_filename);
    let mut posts_db = get_db(&app.config().posts_db_filename);
    //let mut inbox_db = get_db(&app.config.inbox_db_filename);

    // get new posts and check for post and comment matches
    let posts = app.get_posts();
    let count = cmp::min(posts.len(), app.config().hot_take as usize);
    for json in &posts[0..count] {
        let post = Post::new(&json["data"].to_string(), &app)?;
        if post.is_match()? && !posts_db.contains(&post.id) {
            println!("Replying to post {}", post.id);
            posts_db.insert(post.id.to_string());
            if let Err(e) = app.reply(&post.id) {
                println!("Error replying to post {}: {}", post.id, e);
            }
        }
        post.get_matching_comments()
            .filter(|id| !comments_db.contains(id))
            .collect::<Vec<_>>()
            .iter_mut()
            .for_each(|id| {
                println!("Replying to comment {}", id);
                comments_db.insert(id.to_string());
                if let Err(e) = app.reply(id) {
                    println!("Error replying to comment {}: {}", id, e);
                };
            });
    }

    write_db(&app.config().posts_db_filename, &posts_db);
    write_db(&app.config().comments_db_filename, &comments_db);
    //write_db(&app.config.inbox_db_filename, &inbox_db);

    Ok(())
}

#[cfg(test)]
mod test;
