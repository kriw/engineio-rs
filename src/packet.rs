/// Reference https://github.com/socketio/engine.io-protocol
use crate::socket::SID;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum PollingType {
    JSONP,
    XHR,
}

#[derive(Debug, Clone)]
pub enum Transport {
    WebSocket,
    Polling(PollingType),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PacketType {
    Open = 0,
    Close,
    Ping,
    Pong,
    Message,
    Upgrade,
    Noop,
}

impl From<String> for PacketType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "0" => Self::Open,
            "1" => Self::Close,
            "2" => Self::Ping,
            "3" => Self::Pong,
            "4" => Self::Message,
            "5" => Self::Upgrade,
            _ => Self::Noop,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WelcomeMessage {
    sid: String,
    upgrades: Vec<String>,
    ping_interval: u32,
    ping_timeout: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub typ: PacketType,
    pub message: String,
}

impl Packet {
    pub fn open(sid: SID) -> Self {
        let welcome = WelcomeMessage {
            sid,
            upgrades: Vec::new(),
            ping_interval: 25000,
            ping_timeout: 5000,
        };
        let message = serde_json::to_string(&welcome).unwrap();
        Self {
            typ: PacketType::Open,
            message,
        }
    }

    pub fn close() -> Self {
        unimplemented!()
    }

    pub fn ping() -> Self {
        Self {
            typ: PacketType::Ping,
            message: "probe".to_string(),
        }
    }

    pub fn pong() -> Self {
        Self {
            typ: PacketType::Pong,
            message: "probe".to_string(),
        }
    }

    pub fn message(message: &str) -> Self {
        Self {
            typ: PacketType::Message,
            message: message.to_string(),
        }
    }

    pub fn upgrade() -> Self {
        unimplemented!()
    }

    pub fn noop() -> Self {
        Self {
            typ: PacketType::Noop,
            message: String::new(),
        }
    }

    pub fn encode(&self) -> String {
        (self.typ as i32).to_string() + &self.message
    }

    pub fn decode(msg: &str) -> Self {
        let (event_str, msg) = msg.split_at(1);
        let typ = PacketType::from(event_str.to_string());
        Self {
            typ,
            message: msg.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Payload {
    packets: Vec<Packet>,
}

impl Payload {
    pub fn encode(&self) -> String {
        let s = self
            .packets
            .iter()
            .map(|p| p.encode())
            .fold(String::new(), |x, y| x + &y);
        format!("{}:{}", s.len(), s)
    }
}

impl From<Vec<Packet>> for Payload {
    fn from(packets: Vec<Packet>) -> Self {
        Self { packets }
    }
}

#[cfg(test)]
mod test {}
