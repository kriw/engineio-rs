use std::collections::HashMap;
use std::marker::Sync;

use crate::packet::{Packet, SID};
use crate::socket::Socket;

use log::{debug, trace};
use serde::Deserialize;
use warp::filters::ws::WebSocket as WSFilter;
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

#[derive(Deserialize, Debug, Clone)]
pub struct QueryParam {
    sid: Option<SID>,
    transport: Option<String>,
}

pub struct Server<W, C>
where
    W: WSEngine + Sync + 'static,
    C: CORSMiddleware + Sync + 'static,
{
    ws: W,
    cors_middleware: Option<C>,
    clients: HashMap<SID, Socket>,
}

impl<W, C> Server<W, C>
where
    W: WSEngine + Sync + 'static,
    C: CORSMiddleware + Sync + 'static,
{
    pub async fn listen() {
        let handle_polling_get = warp::path("engine.io")
            .and(warp::path::end())
            .and(warp::query::<QueryParam>())
            .map(|param: QueryParam| {
                debug!("http GET message: {:?}", param);
                // TODO Handle Request
                Packet::open().encode()
            });
        let handle_polling_post = warp::path("engine.io")
            .and(warp::path::end())
            .and(warp::body::content_length_limit(1024 * 32))
            .and(warp::body::bytes())
            .map(|bytes: bytes::Bytes| {
                debug!("http POST message: {:?}", bytes);
                // TODO Handle Request
                Packet::open().encode()
            });

        let handle_ws = warp::path("engine.io")
            .and(warp::path::end())
            .and(warp::query::<QueryParam>())
            .and(warp::ws())
            .map(|param: QueryParam, ws: warp::ws::Ws| {
                println!("ws: {:?}", ws);
                ws.on_upgrade(|socket| Self::on_ws_connected(socket, param))
            });
        warp::serve(handle_ws.or(handle_polling_post).or(handle_polling_get))
            .run(([0, 0, 0, 0], 3030))
            .await;
    }

    async fn on_ws_connected(ws: WebSocket, param: QueryParam) {
        println!("on upgrade {:?}", ws);
        Self::on_websocket(ws, param).await;
    }

    async fn on_websocket(ws: WSFilter, param: QueryParam) {
        println!("Param: {:?}", param);
        // TODO Get client sid from query params
        let sock = if let Some(_sid) = param.sid {
            // TODO Get Socket by sid
            // unimplemented!()
            Socket::new(ws)
        } else {
            Self::handshake(ws).await
        };
        sock.run_ws().await;
    }

    async fn handshake(ws: WSFilter) -> Socket {
        trace!("handshake");
        // TODO Get transport from query params
        // TODO Check binary is supported (binary mode if b64 is set true)

        let mut ret = Socket::new(ws);
        ret.on_open().await;

        // TODO Emit `open` event
        // TODO Emit `connection` event
        ret
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
