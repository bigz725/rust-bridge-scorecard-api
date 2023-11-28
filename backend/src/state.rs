use mongodb::Client;

#[derive(Clone)]
pub struct AppState {
    pub mongodb_client: Client,
}