use crate::transports::{Result, Transport};

use async_trait::async_trait;
use futures::stream::{SplitSink, SplitStream};
use futures::StreamExt;
use warp::filters::ws::{Message, WebSocket as WS};

pub struct WebSocket {
    tx: SplitSink<WS, Message>,
    rx: SplitStream<WS>,
}

impl WebSocket {
    pub fn new(ws: WS) -> Self {
        let (tx, rx) = ws.split();
        Self { tx, rx }
    }
}

#[async_trait]
impl Transport for WebSocket {
    async fn send_message(&self, message: String) -> Result {
        Ok(())
    }
    async fn send_ping(&self) -> Result {
        Ok(())
    }
    async fn send_pong(&self) -> Result {
        Ok(())
    }
    async fn send_close(&self) -> Result {
        Ok(())
    }
}
