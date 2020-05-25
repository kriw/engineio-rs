use std::collections::HashMap;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::marker::Sync;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::packet::{Packet, Payload};
use crate::socket::{Socket, SID};
use crate::util;

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

type Clients = Arc<Mutex<HashMap<SID, util::BiChan<String, String>>>>;

#[derive(Debug)]
pub struct Server<W, C>
where
    W: WSEngine + Sync + Send + 'static,
    C: CORSMiddleware + Sync + Send + 'static,
{
    clients: Clients,
    phantom_ws: PhantomData<W>,
    phantom_cors: PhantomData<C>,
}

impl<W, C> Clone for Server<W, C>
where
    W: WSEngine + Sync + Send + 'static,
    C: CORSMiddleware + Sync + Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            clients: self.clients.clone(),
            phantom_ws: PhantomData,
            phantom_cors: PhantomData,
        }
    }
}

impl<W, C> Default for Server<W, C>
where
    W: WSEngine + Sync + Send + 'static,
    C: CORSMiddleware + Sync + Send + 'static,
{
    fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::default())),
            phantom_ws: PhantomData,
            phantom_cors: PhantomData,
        }
    }
}

impl<W, C> Server<W, C>
where
    W: WSEngine + Sync + Send + 'static,
    C: CORSMiddleware + Sync + Send + 'static,
{
    pub async fn listen() {
        let handler = {
            let server = Self::default();
            let server = warp::any().map(move || server.clone());
            let handle_polling_get = warp::path("engine.io")
                .and(warp::path::end())
                .and(server.clone())
                .and(warp::query::<QueryParam>())
                .and_then(Self::on_get);
            let handle_polling_post = warp::path("engine.io")
                .and(warp::path::end())
                .and(server.clone())
                .and(warp::query::<QueryParam>())
                .and(warp::body::content_length_limit(1024 * 32))
                .and(warp::body::bytes())
                .and_then(Self::on_post);

            let handle_ws = warp::path("engine.io")
                .and(warp::path::end())
                .and(warp::query::<QueryParam>())
                .and(warp::ws())
                .and(server)
                .map(|param: QueryParam, ws: warp::ws::Ws, server: Self| {
                    println!("ws: {:?}", ws);
                    ws.on_upgrade(move |socket| server.on_ws_connected(socket, param))
                });
            handle_ws.or(handle_polling_post).or(handle_polling_get)
        };
        warp::serve(handler).run(([0, 0, 0, 0], 3030)).await;
    }

    async fn on_get(self, param: QueryParam) -> Result<String, Infallible> {
        Ok(self.on_request(param, None, false).await)
    }

    async fn on_post(self, param: QueryParam, bytes: bytes::Bytes) -> Result<String, Infallible> {
        Ok(self.on_request(param, Some(bytes), true).await)
    }

    async fn on_request(
        self,
        param: QueryParam,
        data: Option<bytes::Bytes>,
        is_post: bool,
    ) -> String {
        debug!(
            "http message: {:?}, data: {:?}, is_post: {}",
            param, data, is_post
        );
        // TODO Handle Request
        if let Some(_sid) = param.sid {
            if is_post {
                Payload::from(vec![Packet::message("po")]).encode()
            } else {
                Payload::from(vec![Packet::noop()]).encode()
            }
        } else {
            Payload::from(vec![Packet::open("todo".to_string())]).encode()
        }
    }

    async fn on_ws_connected(self, ws: WebSocket, param: QueryParam) {
        println!("on upgrade {:?}", ws);
        println!("Param: {:?}", param);
        // TODO Get client sid from query params
        let sock = if let Some(_sid) = param.sid {
            // TODO Get Socket by sid
            unimplemented!()
        // Socket::new(ws)
        } else {
            self.handshake(ws).await
        };
        sock.run_ws().await;
    }

    async fn handshake(self, ws: WSFilter) -> Socket {
        trace!("handshake");
        // TODO Get transport from query params
        // TODO Check binary is supported (binary mode if b64 is set true)

        let (tx, rx) = util::BiChan::new();
        let mut ret = Socket::new(rx, ws);
        self.clients.lock().await.insert(ret.sid(), tx);
        debug!("#Client: {:?}", self.clients.lock().await.len());
        ret.on_open().await;

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

    pub fn handle_request(&self) {
        // Verify the request
        // Exec callback function with corresponding sid if the callback exists
        unimplemented!()
    }
}
