use engineio_rs::server::*;

#[tokio::main]
async fn main() {
    simple_logger::init().unwrap();
    Server::<Fake, Fake>::listen().await;
}
