pub mod polling;
pub mod polling_jsonp;
pub mod websocket;

use async_trait::async_trait;

#[derive(Debug)]
pub enum TransportError {
    WebSocketError(warp::Error),
}

pub type Result = std::result::Result<(), TransportError>;

#[async_trait]
pub trait Transport {
    async fn send_message(&self, message: String) -> Result;
    async fn send_ping(&self) -> Result;
    async fn send_pong(&self) -> Result;
    async fn send_close(&self) -> Result;
}
