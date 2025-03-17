use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_write_style_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to the Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}