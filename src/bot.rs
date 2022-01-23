use tokio_tungstenite::{client_async_tls};
use url::Url;

struct Bot {
    ws: WebSocketStream
}

impl Bot {
    fn new(token: &str) -> Bot {
        let (mut ws_stream, _) = connect_async_tls(Url::parse("wss://gateway.discord.gg/?v=9&encoding=json")).await?;
    }
}