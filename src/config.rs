use config::Config;
use crate::models::credentials::Credentials;

pub fn load_config() -> Result<Config, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml").required(true))
        .build()?;

    Ok(settings)
}

pub fn get_credentials(config: &Config) -> Result<Credentials, config::ConfigError> {
    config.get::<Credentials>("credentials")
}
