use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = TcpListener::bind("127.0.0.1:8001").unwrap();
    run(host)?.await
}
