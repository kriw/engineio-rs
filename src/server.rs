use std::collections::HashMap;

use crate::client::Client;

pub enum VerifyError {
    UnknownTransport,
    UnknownSID,
    BadHandshakeMethod,
}

pub type VerifyResult = Result<(), VerifyError>;

pub type SID = String;

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
    W: WSEngine,
    C: CORSMiddleware,
{
    ws: W,
    cors_middleware: Option<C>,
    clients: HashMap<SID, Client>,
}

impl<W, C> Server<W, C>
where
    W: WSEngine,
    C: CORSMiddleware,
{
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
