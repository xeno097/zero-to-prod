use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero_to_prod::configuration::get_configuration;
use zero_to_prod::startup::run;
use zero_to_prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    let connection = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection)?.await
}
