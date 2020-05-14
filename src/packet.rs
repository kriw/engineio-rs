/// Reference https://github.com/socketio/engine.io-protocol

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

#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    Open = 0,
    Close,
    Ping,
    Pong,
    Message,
    Upgrade,
    Noop,
}

#[derive(Debug, Clone)]
pub struct Packet {
    typ: PacketType,
    message: String,
}

impl Packet {
    pub fn open() -> Self {
        unimplemented!()
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
}

#[derive(Debug, Clone)]
pub struct Payload {
    packets: Vec<Packet>,
}
