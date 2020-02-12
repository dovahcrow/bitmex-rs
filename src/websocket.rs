mod command;
mod message;
mod topic;

pub use self::command::Command;
pub use self::message::Message as BitMEXWsMessage;
pub use self::message::{
    Action, CancelAllAfterMessage, ErrorMessage, InfoMessage, Limit, SuccessMessage, TableFilter,
    TableMessage,
};
pub use self::topic::Topic;
use crate::consts::WS_URL;
use crate::BitMEX;
use failure::Fallible;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use log::trace;
use pin_project::pin_project;
use serde_json::{from_str, to_string};
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message as WSMessage;
use url::Url;

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl BitMEX {
    pub async fn websocket(&self) -> Fallible<BitMEXWebsocket> {
        let (stream, _) = connect_async(Url::parse(&WS_URL).unwrap()).await?;
        Ok(BitMEXWebsocket::new(stream))
    }
}

#[pin_project]
pub struct BitMEXWebsocket {
    #[pin]
    inner: WSStream,
}

impl BitMEXWebsocket {
    fn new(ws: WSStream) -> Self {
        Self { inner: ws }
    }
}

impl Sink<Command> for BitMEXWebsocket {
    type Error = failure::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.inner.poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(self: Pin<&mut Self>, item: Command) -> Result<(), Self::Error> {
        let this = self.project();
        let command = match &item {
            &Command::Ping => "ping".to_string(),
            command => to_string(command)?,
        };
        trace!("Sending '{}' through websocket", command);
        Ok(this.inner.start_send(WSMessage::Text(command))?)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.inner.poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.inner.poll_close(cx).map_err(|e| e.into())
    }
}

impl Stream for BitMEXWebsocket {
    type Item = Fallible<BitMEXWsMessage>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let poll = this.inner.poll_next(cx);
        match poll {
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(Some(Ok(m))) => Poll::Ready(Some(Ok(parse_message(m)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn parse_message(msg: WSMessage) -> BitMEXWsMessage {
    match msg {
        WSMessage::Text(message) => match message.as_str() {
            "pong" => BitMEXWsMessage::Pong,
            others => match from_str(others) {
                Ok(r) => r,
                Err(_) => unreachable!("Received message from BitMEX: '{}'", others),
            },
        },
        _ => unreachable!(),
    }
}
