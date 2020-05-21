use std::marker::Unpin;

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use log::{debug, trace};
use warp::filters::ws::{Message, WebSocket};

// Transport traitをつかった実装になおす (JSONP, XHRに対応する。)
pub struct Socket {
    tx: SplitSink<WebSocket, Message>,
    rx: SplitStream<WebSocket>,
}

impl Socket {
    pub fn new(ws: WebSocket) -> Self {
        let (tx, rx) = ws.split();
        Self { tx, rx }
    }

    pub async fn on_open(&mut self) {
        trace!("on open");
        // TODO Replace with actual value
        let message = Message::text(
            r#"0{"sid":"sSRZaN1iQJy4BS31AAAE","upgrades":[],"pingInterval":25000,"pingTimeout":5000}"#,
        );
        self.tx.send(message).await;
    }

    pub async fn on_message(&mut self, msg: &Message) {
        debug!("message: {:?}", msg);
        let message = Message::text(r#"4{"message":"hello"}"#);
        self.tx.send(message).await.unwrap();
    }

    pub async fn on_close(&mut self) {
        unimplemented!()
    }

    pub async fn on_ping(&mut self) {
        unimplemented!()
    }

    pub async fn on_pong(&mut self) {
        unimplemented!()
    }

    pub async fn on_upgrade(&mut self) {
        unimplemented!()
    }

    pub async fn on_noop(&mut self) {
        // nop
    }

    pub async fn run_ws(mut self) {
        // TODO Call handshake if sid is not set
        while let Some(Ok(result)) = self.rx.next().await {
            // TODO Decode packet
            // TODO Invoke proper functions for each event
            self.on_message(&result).await;
            // self.on_close().await;
            // self.on_ping().await;
            // self.on_pong().await;
            // self.on_upgrade().await;
        }
    }
}
