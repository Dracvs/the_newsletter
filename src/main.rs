use std::net::TcpListener;
use the_newsletter::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Couldn't bind port");
    run(listener)?.await
}
