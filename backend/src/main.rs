use backend::{configuration::get_configuration, startup::Application};
use backend::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    dotenv::dotenv().ok();
    tracing::info!("Starting server...");

    let application = Application::build(configuration).await.unwrap();
    application.run_until_stopped().await;
}

