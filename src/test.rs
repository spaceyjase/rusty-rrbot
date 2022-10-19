use crate::config::Config;
use crate::reddit::post::{Post, RE};
use crate::reddit::RedditApp;
use failure::Error;
use orca::data::Comment;
use orca::data::Listing;

struct RedditMock {
    pub comments: Listing<Comment>,
}

impl RedditMock {
    pub fn new(comments: Option<Listing<Comment>>) -> RedditMock {
        let comments = comments.unwrap_or_default();
        RedditMock { comments }
    }
}

impl RedditApp for RedditMock {
    fn get_comment_tree(self: &RedditMock, _post_id: &str) -> Result<Listing<Comment>, Error> {
        Ok(self.comments.clone())
    }
    fn get_posts(&self) -> std::vec::Vec<serde_json::Value> {
        Vec::new()
    }
    fn reply(&self, _: &str) -> std::result::Result<(), failure::Error> {
        todo!()
    }
}

impl Default for RedditMock {
    fn default() -> Self {
        Option::<Self>::None.unwrap()
    }
}

#[test]
fn test_config_parse() {
    let config = Config::new("test_config.json").unwrap();

    assert_eq!(config.client_id, "client_id");
    assert_eq!(config.client_secret, "client_secret");
    assert_eq!(config.username, "username");
    assert_eq!(config.password, "password");
    assert_eq!(config.hot_take, 50);
    assert_eq!(config.inbox_db_filename, "inbox.db");
    assert_eq!(config.posts_db_filename, "posts.db");
    assert_eq!(config.comments_db_filename, "comments.db");
    assert_eq!(config.sub, "sub");
    assert_eq!(config.monitor_only, true);
}

#[test]
fn test_post_parse() {
    let json = r#"
    {
      "all_awardings": [],
      "allow_live_comments": false,
      "approved_at_utc": null,
      "approved_by": null,
      "archived": false,
      "author": "AutoModerator",
      "author_flair_background_color": null,
      "author_flair_css_class": null,
      "author_flair_richtext": [],
      "author_flair_template_id": null,
      "author_flair_text": null,
      "author_flair_text_color": null,
      "author_flair_type": "text",
      "author_fullname": "t2_6l4z3",
      "author_is_blocked": false,
      "author_patreon_flair": false,
      "author_premium": true,
      "awarders": [],
      "banned_at_utc": null,
      "banned_by": null,
      "can_gild": false,
      "can_mod_post": false,
      "category": null,
      "clicked": false,
      "content_categories": null,
      "contest_mode": false,
      "created": 1637150412.0,
      "created_utc": 1637150412.0,
      "discussion_type": null,
      "distinguished": null,
      "domain": "self.bodyweightfitness",
      "downs": 0,
      "edited": false,
      "gilded": 0,
      "gildings": {},
      "hidden": false,
      "hide_score": false,
      "id": "qvxrbp",
      "is_created_from_ads_ui": false,
      "is_crosspostable": false,
      "is_meta": false,
      "is_original_content": false,
      "is_reddit_media_domain": false,
      "is_robot_indexable": true,
      "is_self": true,
      "is_video": false,
      "likes": null,
      "link_flair_background_color": "",
      "link_flair_css_class": null,
      "link_flair_richtext": [],
      "link_flair_text": null,
      "link_flair_text_color": "dark",
      "link_flair_type": "text",
      "locked": false,
      "media": null,
      "media_embed": {},
      "media_only": false,
      "mod_note": null,
      "mod_reason_by": null,
      "mod_reason_title": null,
      "mod_reports": [],
      "name": "t3_qvxrbp",
      "no_follow": false,
      "num_comments": 7,
      "num_crossposts": 0,
      "num_reports": null,
      "over_18": false,
      "parent_whitelist_status": "all_ads",
      "permalink": "/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "pinned": false,
      "pwls": 6,
      "quarantine": false,
      "removal_reason": null,
      "removed_by": null,
      "removed_by_category": null,
      "report_reasons": null,
      "saved": false,
      "score": 1,
      "secure_media": null,
      "secure_media_embed": {},
      "selftext": " Welcome to the [r/bodyweightfitness](https://www.reddit.com/r/bodyweightfitness/) daily discussion thread!\n\nFeel free to post beginner questions or just about anything that's on your mind related to fitness!\n\n**Reminders:**\n\n* Read the [FAQ](http://www.reddit.com/r/bodyweightfitness/wiki/faq) as your question may be answered there already.\n* If you're unsure how to start training, try the [BWF Primer Routine](https://www.reddit.com/r/bodyweightfitness/comments/kofo8l/bwf_primer_buildup_community_event_day_1_happy/), check out our [Recommended Routine](https://www.reddit.com/r/bodyweightfitness/wiki/kb/recommended_routine), or our more skills based routine: [Move](https://www.reddit.com/r/bodyweightfitness/wiki/move).\n* Even though the rules are relaxed here, asking for medical advice is still not allowed.\n\nJoin our live conversations on [Discord](https://discord.gg/bwf)! We're also on [Facebook](https://www.facebook.com/redditbwf), [Instagram](https://www.instagram.com/redditbwf), and [Twitter](https://www.twitter.com/redditbwf)!\n\nIf you'd like to look at previous Discussion threads, [click here.](https://www.reddit.com/r/bodyweightfitness/search?q=Daily+Discussion+and+Beginner&amp;sort=new&amp;restrict_sr=on)",
      "selftext_html": "&lt;!-- SC_OFF --&gt;&lt;div class=\"md\"&gt;&lt;p&gt;Welcome to the &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/\"&gt;r/bodyweightfitness&lt;/a&gt; daily discussion thread!&lt;/p&gt;\n\n&lt;p&gt;Feel free to post beginner questions or just about anything that&amp;#39;s on your mind related to fitness!&lt;/p&gt;\n\n&lt;p&gt;&lt;strong&gt;Reminders:&lt;/strong&gt;&lt;/p&gt;\n\n&lt;ul&gt;\n&lt;li&gt;Read the &lt;a href=\"http://www.reddit.com/r/bodyweightfitness/wiki/faq\"&gt;FAQ&lt;/a&gt; as your question may be answered there already.&lt;/li&gt;\n&lt;li&gt;If you&amp;#39;re unsure how to start training, try the &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/comments/kofo8l/bwf_primer_buildup_community_event_day_1_happy/\"&gt;BWF Primer Routine&lt;/a&gt;, check out our &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/wiki/kb/recommended_routine\"&gt;Recommended Routine&lt;/a&gt;, or our more skills based routine: &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/wiki/move\"&gt;Move&lt;/a&gt;.&lt;/li&gt;\n&lt;li&gt;Even though the rules are relaxed here, asking for medical advice is still not allowed.&lt;/li&gt;\n&lt;/ul&gt;\n\n&lt;p&gt;Join our live conversations on &lt;a href=\"https://discord.gg/bwf\"&gt;Discord&lt;/a&gt;! We&amp;#39;re also on &lt;a href=\"https://www.facebook.com/redditbwf\"&gt;Facebook&lt;/a&gt;, &lt;a href=\"https://www.instagram.com/redditbwf\"&gt;Instagram&lt;/a&gt;, and &lt;a href=\"https://www.twitter.com/redditbwf\"&gt;Twitter&lt;/a&gt;!&lt;/p&gt;\n\n&lt;p&gt;If you&amp;#39;d like to look at previous Discussion threads, &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/search?q=Daily+Discussion+and+Beginner&amp;amp;sort=new&amp;amp;restrict_sr=on\"&gt;click here.&lt;/a&gt;&lt;/p&gt;\n&lt;/div&gt;&lt;!-- SC_ON --&gt;",
      "send_replies": true,
      "spoiler": false,
      "stickied": true,
      "subreddit": "bodyweightfitness",
      "subreddit_id": "t5_2tf0a",
      "subreddit_name_prefixed": "r/bodyweightfitness",
      "subreddit_subscribers": 2289660,
      "subreddit_type": "public",
      "suggested_sort": "new",
      "thumbnail": "",
      "title": "BWF Daily Discussion and Beginner/RR Questions Thread for 2021-11-17",
      "top_awarded_type": null,
      "total_awards_received": 0,
      "treatment_tags": [],
      "ups": 1,
      "upvote_ratio": 1.0,
      "url": "https://www.reddit.com/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "user_reports": [],
      "view_count": null,
      "visited": false,
      "whitelist_status": "all_ads",
      "wls": 6
    }"#;
    let mock = RedditMock::new(Option::None);
    let post = Post::new(json, &mock).unwrap();
    assert_eq!(post.id, "qvxrbp");
}

#[test]
fn test_matching_post_false() {
    let json = r#"
    {
      "all_awardings": [],
      "allow_live_comments": false,
      "approved_at_utc": null,
      "approved_by": null,
      "archived": false,
      "author": "AutoModerator",
      "author_flair_background_color": null,
      "author_flair_css_class": null,
      "author_flair_richtext": [],
      "author_flair_template_id": null,
      "author_flair_text": null,
      "author_flair_text_color": null,
      "author_flair_type": "text",
      "author_fullname": "t2_6l4z3",
      "author_is_blocked": false,
      "author_patreon_flair": false,
      "author_premium": true,
      "awarders": [],
      "banned_at_utc": null,
      "banned_by": null,
      "can_gild": false,
      "can_mod_post": false,
      "category": null,
      "clicked": false,
      "content_categories": null,
      "contest_mode": false,
      "created": 1637150412.0,
      "created_utc": 1637150412.0,
      "discussion_type": null,
      "distinguished": null,
      "domain": "self.bodyweightfitness",
      "downs": 0,
      "edited": false,
      "gilded": 0,
      "gildings": {},
      "hidden": false,
      "hide_score": false,
      "id": "qvxrbp",
      "is_created_from_ads_ui": false,
      "is_crosspostable": false,
      "is_meta": false,
      "is_original_content": false,
      "is_reddit_media_domain": false,
      "is_robot_indexable": true,
      "is_self": true,
      "is_video": false,
      "likes": null,
      "link_flair_background_color": "",
      "link_flair_css_class": null,
      "link_flair_richtext": [],
      "link_flair_text": null,
      "link_flair_text_color": "dark",
      "link_flair_type": "text",
      "locked": false,
      "media": null,
      "media_embed": {},
      "media_only": false,
      "mod_note": null,
      "mod_reason_by": null,
      "mod_reason_title": null,
      "mod_reports": [],
      "name": "t3_qvxrbp",
      "no_follow": false,
      "num_comments": 7,
      "num_crossposts": 0,
      "num_reports": null,
      "over_18": false,
      "parent_whitelist_status": "all_ads",
      "permalink": "/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "pinned": false,
      "pwls": 6,
      "quarantine": false,
      "removal_reason": null,
      "removed_by": null,
      "removed_by_category": null,
      "report_reasons": null,
      "saved": false,
      "score": 1,
      "secure_media": null,
      "secure_media_embed": {},
      "selftext": "",
      "send_replies": true,
      "spoiler": false,
      "stickied": true,
      "subreddit": "bodyweightfitness",
      "subreddit_id": "t5_2tf0a",
      "subreddit_name_prefixed": "r/bodyweightfitness",
      "subreddit_subscribers": 2289660,
      "subreddit_type": "public",
      "suggested_sort": "new",
      "thumbnail": "",
      "title": "BWF Daily Discussion and Beginner/RR Questions Thread for 2021-11-17",
      "top_awarded_type": null,
      "total_awards_received": 0,
      "treatment_tags": [],
      "ups": 1,
      "upvote_ratio": 1.0,
      "url": "https://www.reddit.com/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "user_reports": [],
      "view_count": null,
      "visited": false,
      "whitelist_status": "all_ads",
      "wls": 6
    }"#;
    let mock = RedditMock::new(Option::None);
    let post = Post::new(json, &mock).unwrap();
    assert_eq!(post.is_match().unwrap_or(false), false);
}

#[test]
fn test_matching_post_true() {
    let json = r#"
    {
      "all_awardings": [],
      "allow_live_comments": false,
      "approved_at_utc": null,
      "approved_by": null,
      "archived": false,
      "author": "AutoModerator",
      "author_flair_background_color": null,
      "author_flair_css_class": null,
      "author_flair_richtext": [],
      "author_flair_template_id": null,
      "author_flair_text": null,
      "author_flair_text_color": null,
      "author_flair_type": "text",
      "author_fullname": "t2_6l4z3",
      "author_is_blocked": false,
      "author_patreon_flair": false,
      "author_premium": true,
      "awarders": [],
      "banned_at_utc": null,
      "banned_by": null,
      "can_gild": false,
      "can_mod_post": false,
      "category": null,
      "clicked": false,
      "content_categories": null,
      "contest_mode": false,
      "created": 1637150412.0,
      "created_utc": 1637150412.0,
      "discussion_type": null,
      "distinguished": null,
      "domain": "self.bodyweightfitness",
      "downs": 0,
      "edited": false,
      "gilded": 0,
      "gildings": {},
      "hidden": false,
      "hide_score": false,
      "id": "qvxrbp",
      "is_created_from_ads_ui": false,
      "is_crosspostable": false,
      "is_meta": false,
      "is_original_content": false,
      "is_reddit_media_domain": false,
      "is_robot_indexable": true,
      "is_self": true,
      "is_video": false,
      "likes": null,
      "link_flair_background_color": "",
      "link_flair_css_class": null,
      "link_flair_richtext": [],
      "link_flair_text": null,
      "link_flair_text_color": "dark",
      "link_flair_type": "text",
      "locked": false,
      "media": null,
      "media_embed": {},
      "media_only": false,
      "mod_note": null,
      "mod_reason_by": null,
      "mod_reason_title": null,
      "mod_reports": [],
      "name": "t3_qvxrbp",
      "no_follow": false,
      "num_comments": 7,
      "num_crossposts": 0,
      "num_reports": null,
      "over_18": false,
      "parent_whitelist_status": "all_ads",
      "permalink": "/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "pinned": false,
      "pwls": 6,
      "quarantine": false,
      "removal_reason": null,
      "removed_by": null,
      "removed_by_category": null,
      "report_reasons": null,
      "saved": false,
      "score": 1,
      "secure_media": null,
      "secure_media_embed": {},
      "selftext": " Welcome to the [r/bodyweightfitness](https://www.reddit.com/r/bodyweightfitness/) daily discussion thread!\n\nFeel free to post beginner questions or just about anything that's on your mind related to fitness!\n\n**Reminders:**\n\n* Read the [FAQ](http://www.reddit.com/r/bodyweightfitness/wiki/faq) as your question may be answered there already.\n* If you're unsure how to start training, try the [BWF Primer Routine](https://www.reddit.com/r/bodyweightfitness/comments/kofo8l/bwf_primer_buildup_community_event_day_1_happy/), check out our [Recommended Routine](https://www.reddit.com/r/bodyweightfitness/wiki/kb/recommended_routine), or our more skills based routine: [Move](https://www.reddit.com/r/bodyweightfitness/wiki/move).\n* Even though the rules are relaxed here, asking for medical advice is still not allowed.\n\nJoin our live conversations on [Discord](https://discord.gg/bwf)! What is the RR? We're also on [Facebook](https://www.facebook.com/redditbwf), [Instagram](https://www.instagram.com/redditbwf), and [Twitter](https://www.twitter.com/redditbwf)!\n\nIf you'd like to look at previous Discussion threads, [click here.](https://www.reddit.com/r/bodyweightfitness/search?q=Daily+Discussion+and+Beginner&amp;sort=new&amp;restrict_sr=on)",
      "send_replies": true,
      "spoiler": false,
      "stickied": true,
      "subreddit": "bodyweightfitness",
      "subreddit_id": "t5_2tf0a",
      "subreddit_name_prefixed": "r/bodyweightfitness",
      "subreddit_subscribers": 2289660,
      "subreddit_type": "public",
      "suggested_sort": "new",
      "thumbnail": "",
      "title": "BWF Daily Discussion and Beginner/RR Questions Thread for 2021-11-17",
      "top_awarded_type": null,
      "total_awards_received": 0,
      "treatment_tags": [],
      "ups": 1,
      "upvote_ratio": 1.0,
      "url": "https://www.reddit.com/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "user_reports": [],
      "view_count": null,
      "visited": false,
      "whitelist_status": "all_ads",
      "wls": 6
    }"#;
    let mock = RedditMock::new(Option::None);
    let post = Post::new(json, &mock).unwrap();
    assert_eq!(post.is_match().unwrap_or(false), true);
}

#[test]
fn test_comment_match_false() {
    let json = r#"
    {
      "all_awardings": [],
      "allow_live_comments": false,
      "approved_at_utc": null,
      "approved_by": null,
      "archived": false,
      "author": "AutoModerator",
      "author_flair_background_color": null,
      "author_flair_css_class": null,
      "author_flair_richtext": [],
      "author_flair_template_id": null,
      "author_flair_text": null,
      "author_flair_text_color": null,
      "author_flair_type": "text",
      "author_fullname": "t2_6l4z3",
      "author_is_blocked": false,
      "author_patreon_flair": false,
      "author_premium": true,
      "awarders": [],
      "banned_at_utc": null,
      "banned_by": null,
      "can_gild": false,
      "can_mod_post": false,
      "category": null,
      "clicked": false,
      "content_categories": null,
      "contest_mode": false,
      "created": 1637150412.0,
      "created_utc": 1637150412.0,
      "discussion_type": null,
      "distinguished": null,
      "domain": "self.bodyweightfitness",
      "downs": 0,
      "edited": false,
      "gilded": 0,
      "gildings": {},
      "hidden": false,
      "hide_score": false,
      "id": "qvxrbp",
      "is_created_from_ads_ui": false,
      "is_crosspostable": false,
      "is_meta": false,
      "is_original_content": false,
      "is_reddit_media_domain": false,
      "is_robot_indexable": true,
      "is_self": true,
      "is_video": false,
      "likes": null,
      "link_flair_background_color": "",
      "link_flair_css_class": null,
      "link_flair_richtext": [],
      "link_flair_text": null,
      "link_flair_text_color": "dark",
      "link_flair_type": "text",
      "locked": false,
      "media": null,
      "media_embed": {},
      "media_only": false,
      "mod_note": null,
      "mod_reason_by": null,
      "mod_reason_title": null,
      "mod_reports": [],
      "name": "t3_qvxrbp",
      "no_follow": false,
      "num_comments": 7,
      "num_crossposts": 0,
      "num_reports": null,
      "over_18": false,
      "parent_whitelist_status": "all_ads",
      "permalink": "/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "pinned": false,
      "pwls": 6,
      "quarantine": false,
      "removal_reason": null,
      "removed_by": null,
      "removed_by_category": null,
      "report_reasons": null,
      "saved": false,
      "score": 1,
      "secure_media": null,
      "secure_media_embed": {},
      "selftext": " Welcome to the [r/bodyweightfitness](https://www.reddit.com/r/bodyweightfitness/) daily discussion thread!\n\nFeel free to post beginner questions or just about anything that's on your mind related to fitness!\n\n**Reminders:**\n\n* Read the [FAQ](http://www.reddit.com/r/bodyweightfitness/wiki/faq) as your question may be answered there already.\n* If you're unsure how to start training, try the [BWF Primer Routine](https://www.reddit.com/r/bodyweightfitness/comments/kofo8l/bwf_primer_buildup_community_event_day_1_happy/), check out our [Recommended Routine](https://www.reddit.com/r/bodyweightfitness/wiki/kb/recommended_routine), or our more skills based routine: [Move](https://www.reddit.com/r/bodyweightfitness/wiki/move).\n* Even though the rules are relaxed here, asking for medical advice is still not allowed.\n\nJoin our live conversations on [Discord](https://discord.gg/bwf)! We're also on [Facebook](https://www.facebook.com/redditbwf), [Instagram](https://www.instagram.com/redditbwf), and [Twitter](https://www.twitter.com/redditbwf)!\n\nIf you'd like to look at previous Discussion threads, [click here.](https://www.reddit.com/r/bodyweightfitness/search?q=Daily+Discussion+and+Beginner&amp;sort=new&amp;restrict_sr=on)",
      "selftext_html": "&lt;!-- SC_OFF --&gt;&lt;div class=\"md\"&gt;&lt;p&gt;Welcome to the &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/\"&gt;r/bodyweightfitness&lt;/a&gt; daily discussion thread!&lt;/p&gt;\n\n&lt;p&gt;Feel free to post beginner questions or just about anything that&amp;#39;s on your mind related to fitness!&lt;/p&gt;\n\n&lt;p&gt;&lt;strong&gt;Reminders:&lt;/strong&gt;&lt;/p&gt;\n\n&lt;ul&gt;\n&lt;li&gt;Read the &lt;a href=\"http://www.reddit.com/r/bodyweightfitness/wiki/faq\"&gt;FAQ&lt;/a&gt; as your question may be answered there already.&lt;/li&gt;\n&lt;li&gt;If you&amp;#39;re unsure how to start training, try the &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/comments/kofo8l/bwf_primer_buildup_community_event_day_1_happy/\"&gt;BWF Primer Routine&lt;/a&gt;, check out our &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/wiki/kb/recommended_routine\"&gt;Recommended Routine&lt;/a&gt;, or our more skills based routine: &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/wiki/move\"&gt;Move&lt;/a&gt;.&lt;/li&gt;\n&lt;li&gt;Even though the rules are relaxed here, asking for medical advice is still not allowed.&lt;/li&gt;\n&lt;/ul&gt;\n\n&lt;p&gt;Join our live conversations on &lt;a href=\"https://discord.gg/bwf\"&gt;Discord&lt;/a&gt;! We&amp;#39;re also on &lt;a href=\"https://www.facebook.com/redditbwf\"&gt;Facebook&lt;/a&gt;, &lt;a href=\"https://www.instagram.com/redditbwf\"&gt;Instagram&lt;/a&gt;, and &lt;a href=\"https://www.twitter.com/redditbwf\"&gt;Twitter&lt;/a&gt;!&lt;/p&gt;\n\n&lt;p&gt;If you&amp;#39;d like to look at previous Discussion threads, &lt;a href=\"https://www.reddit.com/r/bodyweightfitness/search?q=Daily+Discussion+and+Beginner&amp;amp;sort=new&amp;amp;restrict_sr=on\"&gt;click here.&lt;/a&gt;&lt;/p&gt;\n&lt;/div&gt;&lt;!-- SC_ON --&gt;",
      "send_replies": true,
      "spoiler": false,
      "stickied": true,
      "subreddit": "bodyweightfitness",
      "subreddit_id": "t5_2tf0a",
      "subreddit_name_prefixed": "r/bodyweightfitness",
      "subreddit_subscribers": 2289660,
      "subreddit_type": "public",
      "suggested_sort": "new",
      "thumbnail": "",
      "title": "BWF Daily Discussion and Beginner/RR Questions Thread for 2021-11-17",
      "top_awarded_type": null,
      "total_awards_received": 0,
      "treatment_tags": [],
      "ups": 1,
      "upvote_ratio": 1.0,
      "url": "https://www.reddit.com/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "user_reports": [],
      "view_count": null,
      "visited": false,
      "whitelist_status": "all_ads",
      "wls": 6
    }"#;
    let mut listing = Listing::<Comment>::new();
    listing.children.push_back(Comment {
        edited: Option::None,
        id: "cj0z5z".to_string(),
        body: "Hello World".to_string(),
        author: "".to_string(),
        downs: 0,
        is_submitter: false,
        link_id: "t3_qvxrbp".to_string(),
        name: "t1_cj0z5z".to_string(),
        parent_id: "t3_qvxrbp".to_string(),
        replies: Listing::<Comment>::new(),
        score: 0,
        score_hidden: false,
        stickied: false,
        subreddit: "".to_string(),
        ups: 0,
    });
    let mock = RedditMock::new(Option::from(listing));
    let post = Post::new(json, &mock).unwrap();
    let matches = post.get_matching_comments().collect::<Vec<_>>();
    assert_eq!(matches.len(), 0);
}

#[test]
fn test_comment_match_true() {
    let json = r#"
    {
      "all_awardings": [],
      "allow_live_comments": false,
      "approved_at_utc": null,
      "approved_by": null,
      "archived": false,
      "author": "AutoModerator",
      "author_flair_background_color": null,
      "author_flair_css_class": null,
      "author_flair_richtext": [],
      "author_flair_template_id": null,
      "author_flair_text": null,
      "author_flair_text_color": null,
      "author_flair_type": "text",
      "author_fullname": "t2_6l4z3",
      "author_is_blocked": false,
      "author_patreon_flair": false,
      "author_premium": true,
      "awarders": [],
      "banned_at_utc": null,
      "banned_by": null,
      "can_gild": false,
      "can_mod_post": false,
      "category": null,
      "clicked": false,
      "content_categories": null,
      "contest_mode": false,
      "created": 1637150412.0,
      "created_utc": 1637150412.0,
      "discussion_type": null,
      "distinguished": null,
      "domain": "self.bodyweightfitness",
      "downs": 0,
      "edited": false,
      "gilded": 0,
      "gildings": {},
      "hidden": false,
      "hide_score": false,
      "id": "qvxrbp",
      "is_created_from_ads_ui": false,
      "is_crosspostable": false,
      "is_meta": false,
      "is_original_content": false,
      "is_reddit_media_domain": false,
      "is_robot_indexable": true,
      "is_self": true,
      "is_video": false,
      "likes": null,
      "link_flair_background_color": "",
      "link_flair_css_class": null,
      "link_flair_richtext": [],
      "link_flair_text": null,
      "link_flair_text_color": "dark",
      "link_flair_type": "text",
      "locked": false,
      "media": null,
      "media_embed": {},
      "media_only": false,
      "mod_note": null,
      "mod_reason_by": null,
      "mod_reason_title": null,
      "mod_reports": [],
      "name": "t3_qvxrbp",
      "no_follow": false,
      "num_comments": 7,
      "num_crossposts": 0,
      "num_reports": null,
      "over_18": false,
      "parent_whitelist_status": "all_ads",
      "permalink": "/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "pinned": false,
      "pwls": 6,
      "quarantine": false,
      "removal_reason": null,
      "removed_by": null,
      "removed_by_category": null,
      "report_reasons": null,
      "saved": false,
      "score": 1,
      "secure_media": null,
      "secure_media_embed": {},
      "selftext": "",
      "selftext_html": "",
      "send_replies": true,
      "spoiler": false,
      "stickied": true,
      "subreddit": "bodyweightfitness",
      "subreddit_id": "t5_2tf0a",
      "subreddit_name_prefixed": "r/bodyweightfitness",
      "subreddit_subscribers": 2289660,
      "subreddit_type": "public",
      "suggested_sort": "new",
      "thumbnail": "",
      "title": "BWF Daily Discussion and Beginner/RR Questions Thread for 2021-11-17",
      "top_awarded_type": null,
      "total_awards_received": 0,
      "treatment_tags": [],
      "ups": 1,
      "upvote_ratio": 1.0,
      "url": "https://www.reddit.com/r/bodyweightfitness/comments/qvxrbp/bwf_daily_discussion_and_beginnerrr_questions/",
      "user_reports": [],
      "view_count": null,
      "visited": false,
      "whitelist_status": "all_ads",
      "wls": 6
    }"#;

    let mut listing = Listing::<Comment>::new();
    listing.children.push_back(Comment {
        edited: Option::None,
        id: "cj0z5z".to_string(),
        body: "Hello World but what is the Rr?".to_string(),
        author: "".to_string(),
        downs: 0,
        is_submitter: false,
        link_id: "t3_qvxrbp".to_string(),
        name: "t1_cj0z5z".to_string(),
        parent_id: "t3_qvxrbp".to_string(),
        replies: Listing::<Comment>::new(),
        score: 0,
        score_hidden: false,
        stickied: false,
        subreddit: "".to_string(),
        ups: 0,
    });
    let mock = RedditMock::new(Option::from(listing));
    let post = Post::new(json, &mock).unwrap();
    let matches = post.get_matching_comments().collect::<Vec<_>>();
    assert_eq!(matches.len(), 1);
}

#[test]
fn test_regex_match_rr() {
    let query = "rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_is_the_rr() {
    let query = "what is the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_the_rr() {
    let query = "what the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_is_rr() {
    let query = "what is rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_rr_qm() {
    let query = "what rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_rr() {
    let query = "what rr".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_s_the_rr() {
    let query = "what's the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_s_the_rr_quoted() {
    let query = r#""what's the rr?""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_s_the_rr_quoted_text() {
    let query = r#"Sarcasm, "What's the rr?" folks snafu."#.to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_s_the_rr_quoted_text_case() {
    let query = r#""What's the RR?""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_s_the_rr_quoted_text_no_match() {
    let query = r#"When somebody asks, "What is the RR?".""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_is_the_rr_and_match() {
    let query = "I'm new to this sub and I would like to ask for some help: What is the RR and where do I find it? ^".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_is_rr_with_text() {
    let query = "can someone tell me what is rr? have i missed something?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_is_the_rr_with_text() {
    let query = "can someone tell me what is the rr? have i missed something?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_also_what_is_the_rr() {
    let query = "Also, what is rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_is_rr_match() {
    let query = "what is rr".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_wat_rr_match() {
    let query = "wat rr".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_what_is_rr_match_2() {
    let query = "what is rr.".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_rr_match() {
    let query = "RR?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_quote_rr_quote_no_match() {
    let query = r#""rr?""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_what_the_rr_no_match() {
    let query = "I tried to decipher what the RR was asking me to do in a workout...".to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_what_does_the_rr_match() {
    let query = "what does the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_what_is_the_rr_stand_for_match() {
    let query = "what is the rr stand for?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_what_does_the_rr_stand_for_match() {
    let query = "what does the rr stand for?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_what_does_the_rr_mean_match() {
    let query = "what does rr mean?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_foo_define_rr_bar_match() {
    let query = "foo define rr bar".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_define_rr_match() {
    let query = "define rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_define_rr_spacing_match() {
    let query = "define rr    ggsddg".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_text_no_match() {
    let query = "define nothing rr    ggsddg".to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_odd_quote_text_match() {
    let query = "what`s the rr?".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_odd_quote_text_no_match() {
    let query = r#""what`s the rr?""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_unicode_from_test_sub() {
    let query = r#""Yeah but what’s the RR? I should have a reply…""#.to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_unicode_from_test_sub_match() {
    let query = r#""Yeah but what's the RR? I should have a reply…""#.to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_from_test_dumb() {
    let query = "Sorry i may seem dumb but whats the rr".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_from_test_whats_the_rr() {
    let query = "whats the Rr".to_string();
    assert!(RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_quote_from_test_whats_the_rr_no_match() {
    let query = r#""whats the Rr""#.to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_match_quote_from_test_thats_what_the_rr_no_match() {
    let query = r#"That's what the RR, the Primer, the Wiki, and the FAQ are for."#.to_string();
    assert!(!RE.is_match(&query).unwrap());
}

#[test]
fn test_regex_empty_string_doesnt_match() {
    let query: String = "".to_string();
    assert!(!RE.is_match(&query).unwrap());
}
