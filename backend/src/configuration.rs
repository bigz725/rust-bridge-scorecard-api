use mongodb::options::ClientOptions;
use secrecy::Secret;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub redis_uri: Secret<String>,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: Option<String>,
    pub password: Option<Secret<String>>,
    pub host: String,
    pub min_pool_size: Option<u32>,
    pub max_pool_size: Option<u32>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub app_name: Option<String>,
    pub database_name: Option<String>,
    pub tls: bool,
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
    pub async fn with_db(&self) -> ClientOptions {
        let mut client_options = self.without_db().await;
        client_options.default_database = self.database_name.clone();
        client_options
    }

    pub async fn without_db(&self) -> ClientOptions {
        let uri = format!(
            "mongodb://{}:{}/",
                self.host, self.port
        );
        let mut client_options = ClientOptions::parse(uri).await.unwrap();
        client_options.app_name = self.app_name.clone();
        client_options.min_pool_size = self.min_pool_size;
        client_options.max_pool_size = self.max_pool_size;
        client_options.default_database = self.database_name.clone();

        client_options
    }
}

// pub async fn db_conn_simple() -> Client {
//     let uri = std::env::var("MONGODB_URL").unwrap_or_else(|_| "mongodb://localhost".to_string());
//     Client::with_uri_str(uri)
//         .await
//         .expect("Failed to connect to mongodb")
// }