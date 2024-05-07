use sqlx::PgPool;
use std::net::TcpListener;
use the_newsletter::configuration::get_configuration;
use the_newsletter::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we cant read config
    let configuration = get_configuration().expect("Failed to read configuration");
    println!("Configuration loaded");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failes to connect to postgres");

    // We have removed the hard coded 8000 its now coming from the settings
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(&address)?;
    println!("Listening on {}", &address);

    run(listener, connection_pool)?.await
}
