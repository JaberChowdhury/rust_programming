use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub max_connections: u32,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthSettings {
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub auth: AuthSettings,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());

        let builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .set_override("database.url", database_url)?
            .set_override("auth.jwt_secret", jwt_secret)?
            .set_override("server.port", port)?;

        builder.build()?.try_deserialize()
    }
}
