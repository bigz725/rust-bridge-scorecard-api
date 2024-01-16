use mongodb::Client;

#[derive(Clone, Debug)]
pub struct AppState {
    pub mongodb_client: Client,
}