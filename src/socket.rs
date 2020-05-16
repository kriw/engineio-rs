use futures::StreamExt;
use warp::filters::ws::WebSocket;

pub struct Socket {
    pub ws: WebSocket,
}

impl Socket {
    pub async fn run_ws(self) {
        // TODO Call handshake if sid is not set
        let (_, mut rx) = self.ws.split();
        while let Some(result) = rx.next().await {
            // on message
            println!("message: {:?}", result);
        }
    }
}
