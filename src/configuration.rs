#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub application_address: String,
}

impl Settings {
    /// Populate and return Settings struct from config file config.{toml,json}
    ///
    /// # Defaults:
    /// - application_port: 5000
    /// - application_address: ::1
    ///
    /// # Errors
    ///
    /// This function will return an error if the config file is invalid.
    pub fn new() -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder()
            .set_default("application_port", "5000")?
            .set_default("application_address", "::1")?
            .add_source(config::File::with_name("config"))
            .build()?;
        builder.try_deserialize()
    }
}
