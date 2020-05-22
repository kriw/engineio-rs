/// Reference https://github.com/socketio/engine.io-protocol
use log::warn;
use serde::{Deserialize, Serialize};

pub type SID = String;

pub fn generate_sid() -> SID {
    // TODO Generate random string
    warn!("[generate_sid] Not yet implemented");
    "sSRZaN1iQJy4BS31AAAE".to_string()
}

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
    pub fn open() -> Self {
        let welcome = WelcomeMessage {
            sid: generate_sid(),
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

    pub fn message(message: String) -> Self {
        Self {
            typ: PacketType::Message,
            message,
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

#[cfg(test)]
mod test {}
