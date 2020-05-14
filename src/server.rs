use std::collections::HashMap;

use crate::client::Client;
use crate::packet::SID;

use futures::{FutureExt, StreamExt};
use warp::ws::WebSocket;
use warp::Filter;

pub struct Fake {}
impl WSEngine for Fake {}
impl CORSMiddleware for Fake {}

pub enum VerifyError {
    UnknownTransport,
    UnknownSID,
    BadHandshakeMethod,
}

pub type VerifyResult = Result<(), VerifyError>;

// TODO
pub struct Cookie {}

// TODO
pub trait CORSMiddleware {}

pub trait WSEngine {}

pub struct ServerOption<W, C>
where
    W: WSEngine,
    C: CORSMiddleware,
{
    ws: W,
    ping_timeout: u32,    // milliseconds
    ping_interval: u32,   // milliseconds,
    upgrade_timeout: u32, // milliseconds,
    max_http_buffer_size: u32,
    cors_middleware: Option<C>,
    cookie: Option<Cookie>,
    allow_request: bool,
}

pub struct Server<W, C>
where
    W: WSEngine + 'static,
    C: CORSMiddleware + 'static,
{
    ws: W,
    cors_middleware: Option<C>,
    clients: HashMap<SID, Client>,
}

impl<W, C> Server<W, C>
where
    W: WSEngine + 'static,
    C: CORSMiddleware + 'static,
{
    pub async fn listen() {
        let handle_polling = warp::path::end().map(|| "OK polling".to_string());
        let handle_ws = warp::path::end().and(warp::ws()).map(|ws: warp::ws::Ws| {
            println!("ws: {:?}", ws);
            ws.on_upgrade(move |socket| Self::on_connected(socket))
        });
        warp::serve(handle_ws.or(handle_polling))
            .run(([0, 0, 0, 0], 3030))
            .await;
    }

    async fn on_connected(ws: WebSocket) {
        println!("on upgrade {:?}", ws);
        let (tx, mut rx) = ws.split();
        while let Some(result) = rx.next().await {
            println!("message: {:?}", result);
        }
    }

    /// Verify a request
    /// 1. Check transport parameter
    /// 2. Check Origin header
    /// 3. Check sid
    ///     - parameter must have `sid`
    ///     - `transport` parameter must be matched with `transport` value set when connected
    ///     - Method must be GET
    ///     - TODO (when allowRequest is not empty)
    fn verify(&self) -> VerifyResult {
        Ok(())
    }

    /// Close all clients
    pub fn close(&mut self) {
        unimplemented!()
    }

    pub fn handle_request(&mut self) {
        // Verify the request
        // Exec callback function with corresponding sid if the callback exists
        unimplemented!()
    }
}
