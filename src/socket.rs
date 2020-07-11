use std::time::Duration;

use crate::packet::{Packet, PacketType, Payload};
use crate::transports::{polling, websocket, Transport};
use crate::util;

use async_trait::async_trait;
use crossbeam::channel::{select, tick, unbounded};
use log::{debug, error, trace, warn};
use rand::prelude::*;

pub type SID = String;

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate_sid() -> SID {
    let mut rng = rand::thread_rng();
    String::from_utf8(
        BASE_STR
            .as_bytes()
            .choose_multiple(&mut rng, 20)
            .cloned()
            .collect(),
    )
    .unwrap()
}

/// Messages used for communication between `Server` and `Socket`s
#[derive(Debug)]
pub enum Message {
    Packet(Packet),
    Payload(Payload),
    Close,
}

impl Message {
    pub fn to_message(s: &str) -> Option<Self> {
        match Packet::decode(s) {
            Ok(p) => {
                return Some(Message::Packet(p));
            }
            Err(e) => {
                warn!("{:?}", e);
            }
        };
        match Payload::decode(s) {
            Ok(p) => {
                return Some(Message::Payload(p));
            }
            Err(e) => {
                warn!("{:?}", e);
            }
        };
        return None;
    }
}

// TODO Refine error type
pub type Result = std::result::Result<(), String>;

#[async_trait]
pub trait EngineIOSocket {
    async fn on_open(&mut self) -> Result;
    async fn on_message(&mut self, packet: &Packet) -> Result;
    async fn on_ping(&mut self, packet: &Packet) -> Result;
    async fn on_pong(&mut self, packet: &Packet) -> Result;
    async fn on_close(&mut self, packet: &Packet) -> Result;
    async fn on_upgrade(&mut self, packet: &Packet) -> Result;
    async fn run(mut self);
}

#[derive(Debug)]
pub struct Socket<T: Transport> {
    sid: SID,
    transport: T,
    ch: util::BiChan<Message, Message>,
    ping_interval: u64,
    ping_timeout: u64,
}

impl<T: Transport> Socket<T> {
    pub fn new(
        transport: T,
        ch: util::BiChan<Message, Message>,
        ping_interval: u64,
        ping_timeout: u64,
    ) -> Self {
        let sid = generate_sid();
        debug!("sid: {:?}", sid);
        Self {
            transport,
            sid,
            ch,
            ping_interval,
            ping_timeout,
        }
    }

    pub fn sid(&self) -> SID {
        self.sid.clone()
    }

    pub async fn handle_request(&mut self, message: &Message) {
        debug!("incoming message {:?}", message);
        let packets: Vec<Packet> = match message {
            Message::Packet(p) => vec![p.clone()],
            Message::Payload(p) => p.clone().into(),
            Message::Close => unimplemented!(),
        };
        for packet in packets.into_iter() {
            // TODO
            unimplemented!()
            // match packet.typ {
            //     PacketType::Open => {}
            //     PacketType::Ping => self.on_ping(&packet).await,
            //     PacketType::Pong => self.on_pong(&packet).await,
            //     PacketType::Close => self.on_close(&packet).await,
            //     PacketType::Message => self.on_message(&packet).await,
            //     PacketType::Upgrade => self.on_upgrade(&packet).await,
            //     PacketType::Noop => {}
            // };
        }
    }
}

#[async_trait]
impl EngineIOSocket for Socket<websocket::WebSocket> {
    async fn on_open(&mut self) -> Result {
        unimplemented!()
        // let message = Packet::open(self.sid.clone()).encode();
        // trace!("on open: {:?}", message);
        // if let Err(e) = self.transport.send_message(message).await {
        //     error!("{:?}", e);
        // }
    }

    async fn on_message(&mut self, packet: &Packet) -> Result {
        unimplemented!()
        // trace!("on message: {:?}", packet);
        // let message = packet.encode();
        // self.transport.send_message(message).await.unwrap();
    }

    async fn on_close(&mut self, _packet: &Packet) -> Result {
        unimplemented!()
    }

    async fn on_ping(&mut self, packet: &Packet) -> Result {
        unimplemented!()
        // trace!("on ping: {:?}", packet);
        // let message = Packet::pong().encode();
        // if let Err(e) = self.transport.send_message(message).await {
        //     error!("{:?}", e);
        // }
    }

    async fn on_pong(&mut self, _packet: &Packet) -> Result {
        unimplemented!()
    }

    async fn on_upgrade(&mut self, _packet: &Packet) -> Result {
        unimplemented!()
    }

    async fn run(mut self) {
        unimplemented!()
        // let ping_tick = tick(Duration::from_millis(self.ping_interval));
        // let (tx, rx) = unbounded();
        // let reset = move || {
        //     tx.send(());
        // };
        // let ping_timeout = util::resettable_timeout(Duration::from_millis(self.ping_timeout), rx);
        //
        // loop {
        //     select! {
        //         recv(self.ch.rx) -> msg => {
        //             if let Ok(s) = msg {
        //                 self.handle_request(&s);
        //             }
        //         },
        //         recv(ping_tick) -> _ => {
        //             if let Err(err) = self.transport.send_ping().await {
        //                 error!("{:?}", err);
        //             }
        //         },
        //         recv(ping_timeout) -> _ => {
        //             // TODO Close connection
        //             unimplemented!()
        //         },
        //     }
        // }
    }
}

#[async_trait]
impl EngineIOSocket for Socket<polling::Polling> {
    async fn on_open(&mut self) -> Result {
        unimplemented!()
        // let message = Packet::open(self.sid.clone()).encode();
        // trace!("on open: {:?}", message);
        // if let Err(e) = self.transport.send_message(message).await {
        //     error!("{:?}", e);
        // }
    }

    async fn on_message(&mut self, packet: &Packet) -> Result {
        unimplemented!()
        // trace!("on message: {:?}", packet);
        // let message = packet.encode();
        // self.transport.send_message(message).await.unwrap();
    }

    async fn on_close(&mut self, _packet: &Packet) -> Result {
        unimplemented!()
    }

    async fn on_ping(&mut self, packet: &Packet) -> Result {
        unimplemented!()
        // trace!("on ping: {:?}", packet);
        // let message = Packet::pong().encode();
        // if let Err(e) = self.transport.send_message(message).await {
        //     error!("{:?}", e);
        // }
    }

    async fn on_pong(&mut self, _packet: &Packet) -> Result {
        unimplemented!()
    }

    async fn on_upgrade(&mut self, _packet: &Packet) -> Result {
        unimplemented!()
    }

    async fn run(self) {
        unimplemented!()
        // // TODO Call handshake if sid is not set
        // while let Some(Ok(result)) = self.rx.next().await {
        //     debug!("incoming message {:?}", result);
        //     if result.is_close() {
        //         // Close socket
        //         unimplemented!()
        //     }
        //     let packet = Packet::decode(result.to_str().unwrap());
        //     match packet.typ {
        //         PacketType::Open => {}
        //         PacketType::Ping => self.on_ping(&packet).await,
        //         PacketType::Pong => self.on_pong(&packet).await,
        //         PacketType::Close => self.on_close(&packet).await,
        //         PacketType::Message => self.on_message(&packet).await,
        //         PacketType::Upgrade => self.on_upgrade(&packet).await,
        //         PacketType::Noop => {}
        //     };
        // }
    }
}
