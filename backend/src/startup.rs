use tokio::net::TcpListener;
use axum::{middleware, Router};
use axum::routing::IntoMakeService;
use mongodb::Client;
use secrecy::{ExposeSecret, Secret};

use crate::middlewares::request_id::add_session_id;
use crate::{ auth::jwt::Keys, configuration::{DatabaseSettings, Settings}, state::AppState, telemetry::add_trace_layer, web::{routes_hello, routes_login} };

pub struct Application {
    pub port: u16,
    pub service: IntoMakeService<Router>, 
    pub listener: TcpListener,
}

impl Application {

    pub fn new(service: IntoMakeService<Router>, listener: TcpListener) -> Self {
        Self {
            port: listener.local_addr().unwrap().port(),
            service,
            listener,
        }
    }
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let db_conn = get_db_conn(&configuration.database).await;

        let address = format!("{}:{}", configuration.application.host, configuration.application.port);
        let jwt_secret = configuration.application.jwt_secret.clone();

        Ok(
            Self::new(run(db_conn, jwt_secret).await, TcpListener::bind(address).await?)
        )
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) {
        tracing::info!("Listening at {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.service).await.unwrap();
    }
}
pub struct ApplicationBaseUrl(pub String);

async fn run(
    db_conn: Client,
    jwt_secret: Secret<String>,
) -> IntoMakeService<Router> {
    let jwt_bytes = jwt_secret.expose_secret().as_bytes();
    let keys = Keys::new(jwt_bytes);
    let state = AppState {
        mongodb_client: db_conn,
        keys
    };


    let router = Router::new()
    .merge(routes_hello::routes(&state))
    .merge(routes_login::routes())
    .with_state(state);

    add_trace_layer(router)
    .layer(middleware::from_fn(add_session_id))
    .into_make_service()
}

pub async fn get_db_conn(configuration: &DatabaseSettings) -> Client {
    let client = Client::with_options(configuration.with_db().await);
    match client {
        Ok(client) => {
            tracing::info!("Connected to MongoDB at {}", configuration.host);
            client
        }
        Err(e) => {
            tracing::error!("Failed to connect to MongoDB: {:?}", e);
            panic!("Failed to connect to MongoDB: {:?}", e);
        }
    }
}

