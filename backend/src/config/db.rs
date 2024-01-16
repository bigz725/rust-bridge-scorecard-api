use mongodb::{Client, options::ServerAddress};

pub async fn db_conn_simple() -> Client {
    let uri = std::env::var("MONGODB_URL").unwrap_or_else(|_| "mongodb://localhost".to_string());
    Client::with_uri_str(uri).await.expect("Failed to connect to mongodb")
}

#[allow(dead_code)]
pub fn db_conn() -> Client {
    let uri = std::env::var("MONGODB_URL").unwrap_or_else(|_| "mongodb://localhost".to_string());
    let server_addresses = vec![ServerAddress::Tcp { host: uri, port: None}];
    let min_pool_size = dotenv::var("MONGODB_MIN_POOL_SIZE").unwrap_or_else(|_| "1".to_string());
    let max_pool_size = dotenv::var("MONGODB_MAX_POOL_SIZE").unwrap_or_else(|_| "10".to_string());
    let database = dotenv::var("MONGODB_DATABASE").unwrap_or_else(|_| "bridge_scorecard_api".to_string());
    let (min_pool_size_int, max_pool_size_int) = (
        min_pool_size.parse::<u32>().unwrap(),
        max_pool_size.parse::<u32>().unwrap()
    );
    let options = mongodb::options::ClientOptions::builder()
        .min_pool_size(min_pool_size_int)
        .max_pool_size(max_pool_size_int)
        .hosts(server_addresses)
        .default_database(database)
        .build();

    Client::with_options(options).expect("Failed to initialize MongoDB client.")
}