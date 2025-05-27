use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub redis_uri: Secret<String>,
}

//TODO: explore this as a way to handle either URL based connection parameters
// or explicit credentials
// Read this: https://serde.rs/enum-representations.html
//
// #[derive(serde::Deserialize, serde::Serialize, Clone)]
// #[serde(untagged)]
// pub enum DatabaseConnectionType {
//     Url{url: String},
//     Credentials{username: String, password: String, host: String, port: u16},
// }


#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: Option<String>,
    pub password: Option<Secret<String>>,
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub min_pool_size: Option<u32>,
    pub max_pool_size: Option<u32>,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub base_url: String,
    pub hmac_secret: Secret<String>,
    pub jwt_secret: Secret<String>,
}
pub enum Environment {
    Local,
    Staging,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" | "development" => Ok(Self::Local),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(format!("{} is not a valid environment", s)),
        }
    }
    
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let configuration_directory = base_path.join("configuration");
    // Detect the running enviroment
    // Default to local if unspecified
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml", environment.as_str());

    // Initialise our configuration reader
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and
        // '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`”
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub async fn with_db(&self) -> PgConnectOptions {
        self.without_db().await.database(&self.database_name).to_owned()
    }

    pub async fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(self.username.as_deref().unwrap_or("postgres"))
            .password(self.password.as_ref().unwrap().expose_secret())
            .ssl_mode(ssl_mode)
    }
}