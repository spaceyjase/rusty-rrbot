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
    pub fn new(filename: &str) -> Result<Config, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(filename))
            .build()?;

        settings.try_deserialize::<Config>()
    }
}
