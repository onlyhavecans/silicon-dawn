use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub application_address: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .set_default("application_port", "5000")?
            .set_default("application_address", "::1")?
            .add_source(File::with_name("config"))
            .build()?;
        builder.try_deserialize()
    }
}
