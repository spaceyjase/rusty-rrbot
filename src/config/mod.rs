#[derive(serde::Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub hot_take: u8,
    pub inbox_db_filename: String,
    pub posts_db_filename: String,
    pub comments_db_filename: String,
    pub sub: String,
    pub monitor_only: bool,
}

impl Config {
    pub fn new(config: &str) -> Result<Config, config::ConfigError> {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name(config))?;
        settings.try_into()
    }
}
