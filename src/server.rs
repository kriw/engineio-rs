use std::collections::HashMap;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::marker::Sync;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use tokio::sync::Mutex;

use crate::packet::{Packet, Payload};
use crate::socket::{Message, Socket, SID};
use crate::transports::{polling, websocket};
use crate::util;

use futures::StreamExt;
use log::{debug, error, info, trace, warn};
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

type Clients = Arc<Mutex<HashMap<SID, util::BiChan<Message, Message>>>>;

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
        debug!("on_get: {:?}", param);
        Ok(self.on_request(param, None, false).await)
    }

    async fn on_post(self, param: QueryParam, bytes: bytes::Bytes) -> Result<String, Infallible> {
        debug!("on_post: {:?}, bytes: {:?}", param, bytes);
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
        match param.sid {
            Some(ref sid) if is_post => self.handle_xhr_post(sid, data).await,
            Some(ref sid) if !is_post => self.handle_xhr_get(sid).await,
            // Send sid for handshaking
            _ if !is_post => {
                let sid = self.clone().handshake().await;
                if let Some(ref client) = self.clients.lock().await.get(&sid) {
                    let s = match client.rx.recv() {
                        Ok(Message::Packet(s)) => s.encode(),
                        Ok(Message::Payload(s)) => s.encode(),
                        _ => String::new(),
                    };
                    debug!("handshake response {:?}", s);
                    s
                } else {
                    warn!("client of sid ({:?}) is not found", sid);
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    async fn handle_xhr_get(self, sid: &SID) -> String {
        debug!("handle_xhr_get: sid = {:?}", sid);
        if let Some(ref client) = self.clients.lock().await.get(sid) {
            info!("Wait");
            match client.rx.recv() {
                Ok(Message::Packet(s)) => {
                    use std::time;
                    thread::sleep(time::Duration::from_millis(3000));
                    info!("Response {:?}", s);
                    s.encode()
                }
                Ok(Message::Payload(s)) => {
                    use std::time;
                    thread::sleep(time::Duration::from_millis(3000));
                    info!("Response {:?}", s);
                    s.encode()
                }
                Ok(s) => {
                    warn!("Invalid message {:?}", s);
                    String::new()
                }
                Err(e) => {
                    error!("Error on handle_xhr_get {:?}", e);
                    String::new()
                }
            }
        } else {
            warn!("Invalid SID, client is not found");
            String::new()
        }
    }

    async fn handle_xhr_post(self, sid: &SID, data: Option<bytes::Bytes>) -> String {
        if let Some(data) = data {
            if let Some(ref client) = self.clients.lock().await.get(sid) {
                if let Ok(s) = String::from_utf8(data.as_ref().to_vec()) {
                    if let Some(p) = Message::to_message(&s) {
                        if let Err(e) = client.tx.send(p) {
                            error!("{:?}", e)
                        }
                    }
                }
            }
        }
        "ok".to_string()
    }

    async fn on_ws_connected(self, ws: WebSocket, param: QueryParam) {
        // XHRと同様にClientからメッセージを待ちつつ、WSからデータがやって来ればClientに投げる
        println!("on upgrade {:?}", ws);
        println!("Param: {:?}", param);
        // TODO Get client sid from query params
        let sock = if let Some(_sid) = param.sid {
            // TODO Get Socket by sid
            unimplemented!()
        // Socket::new(ws)
        } else {
            self.handshake_ws(ws).await
        };
        unimplemented!()
        // sock.run_ws().await;
    }

    async fn handshake_ws(self, ws: WSFilter) -> Socket<websocket::WebSocket> {
        trace!("handshake");
        // TODO Get transport from query params
        // TODO Check binary is supported (binary mode if b64 is set true)

        unimplemented!()
        // let (ch1, ch2) = util::BiChan::new();
        // let (tx, mut rx) = ws.split();
        // let transport_ws = websocket::WebSocket::new(tx);
        // thread::spawn(async move || {
        //     while let Some(Ok(result)) = rx.next().await {
        //         ch2.tx.send(Message::WebSocket(result));
        //     }
        // });
        // let mut ret = Socket::<websocket::WebSocket>::new(transport_ws, ch1, 1500, 1500);
        //
        // self.clients.lock().await.insert(ret.sid(), ch2);
        // debug!("#Client: {:?}", self.clients.lock().await.len());
        // ret.on_open().await;
        //
        // ret
    }

    async fn handshake(self) -> SID {
        trace!("handshake");
        let (ch1, ch2) = util::BiChan::new();
        let transport = polling::Polling::new();
        let mut ret = Socket::<polling::Polling>::new(transport, ch1, 1500, 1500);

        self.clients.lock().await.insert(ret.sid(), ch2);
        debug!("#Client: {:?}", self.clients.lock().await.len());
        ret.on_open().await;
        ret.sid()
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
