use engineio_rs::server::*;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    Server::<Fake, Fake>::listen().await;
}
