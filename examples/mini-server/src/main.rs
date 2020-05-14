use engineio_rs::server::*;

#[tokio::main]
async fn main() {
    Server::<Fake, Fake>::listen().await;
}
