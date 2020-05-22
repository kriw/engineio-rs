pub mod polling;
pub mod polling_jsonp;
pub mod websocket;

pub trait Transport {
    fn on_request(&mut self, data: &str);
}
