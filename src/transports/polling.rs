use crate::transports::{Result, Transport};

use async_trait::async_trait;

pub struct Polling {}

impl Polling {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Transport for Polling {
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
