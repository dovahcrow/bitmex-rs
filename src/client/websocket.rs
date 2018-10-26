use failure::Error;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::{Future, Poll, StartSend};
use log::trace;
use serde_json::{from_str, to_string};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message;
use url::Url;

use crate::consts::WS_URL;
use crate::model::websocket::{Command, Message as BitMEXWsMessage};
use crate::BitMEX;

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl BitMEX {
    pub fn websocket(&self) -> impl Future<Item = BitMEXWebsocket, Error = Error> {
        connect_async(Url::parse(WS_URL).unwrap()).map(|(stream, _)| stream).from_err().map(BitMEXWebsocket::new)
    }
}

pub struct BitMEXWebsocket {
    inner: WSStream,
}

impl BitMEXWebsocket {
    fn new(ws: WSStream) -> Self {
        Self { inner: ws }
    }
}

impl Sink for BitMEXWebsocket {
    type SinkItem = Command;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        let command = match &item {
            &Command::Ping => "ping".to_string(),
            command => to_string(command)?,
        };
        trace!("Sending '{}' through websocket", command);
        Ok(self.inner.start_send(Message::Text(command))?.map(|_| item))
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(self.inner.poll_complete()?)
    }
}

impl Stream for BitMEXWebsocket {
    type Item = BitMEXWsMessage;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(self.inner.poll().map(|a| a.map(|i| i.map(parse_message)))?)
    }
}

fn parse_message(msg: Message) -> BitMEXWsMessage {
    match msg {
        Message::Text(message) => match message.as_str() {
            "pong" => BitMEXWsMessage::Pong,
            others => match from_str(others) {
                Ok(r) => r,
                Err(_) => unreachable!("Received message from BitMEX: '{}'", others),
            },
        },
        _ => unreachable!(),
    }
}
