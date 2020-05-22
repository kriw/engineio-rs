use crate::packet::{Packet, PacketType};

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use log::{debug, error, trace};
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
        let message = Message::text(Packet::open().encode().as_str());
        trace!("on open: {:?}", message);
        if let Err(e) = self.tx.send(message).await {
            error!("{:?}", e);
        }
    }

    pub async fn on_message(&mut self, packet: &Packet) {
        trace!("on message: {:?}", packet);
        let message = Message::text(packet.encode().as_str());
        self.tx.send(message).await.unwrap();
    }

    pub async fn on_close(&mut self, _packet: &Packet) {
        unimplemented!()
    }

    pub async fn on_ping(&mut self, packet: &Packet) {
        trace!("on ping: {:?}", packet);
        let message = Message::text(Packet::pong().encode().as_str());
        if let Err(e) = self.tx.send(message).await {
            error!("{:?}", e);
        }
    }

    pub async fn on_pong(&mut self, _packet: &Packet) {
        unimplemented!()
    }

    pub async fn on_upgrade(&mut self, _packet: &Packet) {
        unimplemented!()
    }

    pub async fn run_ws(mut self) {
        // TODO Call handshake if sid is not set
        while let Some(Ok(result)) = self.rx.next().await {
            debug!("incoming message {:?}", result);
            if result.is_close() {
                // Close socket
                unimplemented!()
            }
            let packet = Packet::decode(result.to_str().unwrap());
            match packet.typ {
                PacketType::Open => {}
                PacketType::Ping => self.on_ping(&packet).await,
                PacketType::Pong => self.on_pong(&packet).await,
                PacketType::Close => self.on_close(&packet).await,
                PacketType::Message => self.on_message(&packet).await,
                PacketType::Upgrade => self.on_upgrade(&packet).await,
                PacketType::Noop => {}
            };
        }
    }
}
