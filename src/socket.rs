use std::time::Duration;

use crate::packet::Packet;
use crate::transports::Transport;
use crate::util;

use crossbeam::channel::{select, tick, unbounded};
use log::{debug, error, trace};
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

// Transport traitをつかった実装になおす (JSONP, XHRに対応する。)
#[derive(Debug)]
pub struct Socket<T: Transport> {
    sid: SID,
    transport: T,
    ch: util::BiChan<String, String>,
    ping_interval: u64,
    ping_timeout: u64,
}

impl<T: Transport> Socket<T> {
    pub fn new(
        transport: T,
        ch: util::BiChan<String, String>,
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

    pub async fn on_open(&mut self) {
        let message = Packet::open(self.sid.clone()).encode();
        trace!("on open: {:?}", message);
        if let Err(e) = self.transport.send_message(message).await {
            error!("{:?}", e);
        }
    }

    pub async fn on_message(&mut self, packet: &Packet) {
        trace!("on message: {:?}", packet);
        let message = packet.encode();
        self.transport.send_message(message).await.unwrap();
    }

    pub async fn on_close(&mut self, _packet: &Packet) {
        unimplemented!()
    }

    pub async fn on_ping(&mut self, packet: &Packet) {
        trace!("on ping: {:?}", packet);
        let message = Packet::pong().encode();
        if let Err(e) = self.transport.send_message(message).await {
            error!("{:?}", e);
        }
    }

    pub async fn on_pong(&mut self, _packet: &Packet) {
        unimplemented!()
    }

    pub async fn on_upgrade(&mut self, _packet: &Packet) {
        unimplemented!()
    }

    pub async fn run_polling(self) {
        unimplemented!()
    }

    pub async fn run(self) {
        let ping_tick = tick(Duration::from_millis(self.ping_interval));
        let (tx, rx) = unbounded();
        let reset = move || {
            tx.send(());
        };
        let ping_timeout = util::resettable_timeout(Duration::from_millis(self.ping_timeout), rx);

        loop {
            select! {
                recv(self.ch.rx) -> _ => {
                    // Handle Message
                    unimplemented!()
                },
                recv(ping_tick) -> _ => {
                    if let Err(err) = self.transport.send_ping().await {
                        error!("{:?}", err);
                    }
                },
                recv(ping_timeout) -> _ => {
                    // TODO Close connection
                    unimplemented!()
                },
            }
        }
    }

    pub async fn run_ws(self) {
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
