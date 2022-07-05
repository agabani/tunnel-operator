use config::{Config, Environment, File};

#[derive(Clone, serde::Deserialize)]
pub struct Configuration {
    pub http_server: HttpServer,
}

#[derive(Clone, serde::Deserialize)]
pub struct HttpServer {
    pub host: String,
    pub port: u16,
}

/// Loads the configuration from the environment variables and the config file.
///
/// # Errors
///
/// If the configuration file cannot be loaded, an error is returned.
pub fn load(overrides: &[(&str, &str)]) -> crate::Result<Configuration> {
    let mut config_builder = Config::builder()
        .set_default("http_server.host", "127.0.0.1")?
        .set_default("http_server.port", "80")?
        .add_source(File::with_name("config").required(false))
        .add_source(Environment::with_prefix("TUNNEL_OPERATOR").separator("__"));

    for &(key, value) in overrides {
        config_builder = config_builder.set_override(key, value)?;
    }

    config_builder
        .build()?
        .try_deserialize()
        .map_err(Into::into)
}
