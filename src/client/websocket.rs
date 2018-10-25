use failure::Error;
use futures::sink::Sink;
use futures::stream::{SplitSink, SplitStream, Stream};
use futures::Future;
use futures::Poll;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message;
use url::Url;

use crate::error::Result;
use crate::model::ws::Topic;
use crate::BitMEX;

const WS_URL: &'static str = "wss://www.bitmex.com/realtime";

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl BitMEX {
    pub fn websocket(&self) -> impl Future<Item = (BitMEXWsSender, BitMEXWsReceiver), Error = Error> {
        connect_async(Url::parse(WS_URL).unwrap())
            .map(|(stream, _)| stream)
            .from_err()
            .map(|s| s.split())
            .map(|(sink, stream)| (BitMEXWsSender::new(sink), BitMEXWsReceiver::new(stream)))
    }
}

pub struct BitMEXWsSender {
    inner: SplitSink<WSStream>,
}

impl BitMEXWsSender {
    fn new(sink: SplitSink<WSStream>) -> Self {
        Self { inner: sink }
    }

    pub fn subscribe(&mut self, topic: Topic) -> Result<()> {
        let command = json! {
            {
                "op": "subscribe", "args": [topic.to_string()]
            }
        };

        self.inner.start_send(Message::text(command.to_string()))?;
        self.inner.poll_complete()?;
        Ok(())
    }

    pub fn unsubscribe(&mut self, topic: Topic) -> Result<()> {
        let command = json! {
            {
                "op": "unsubscribe", "args": [topic.to_string()]
            }
        };

        self.inner.start_send(Message::text(command.to_string()))?;
        self.inner.poll_complete()?;
        Ok(())
    }
}
pub struct BitMEXWsReceiver {
    inner: SplitStream<WSStream>,
}

impl BitMEXWsReceiver {
    fn new(stream: SplitStream<WSStream>) -> Self {
        Self { inner: stream }
    }
}

impl Stream for BitMEXWsReceiver {
    type Item = WsMessage;
    type Error = Error;
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(self.inner.poll().map(|a| {
            a.map(|i| {
                i.map(|resp| match resp {
                    _ => WsMessage::Ok,
                })
            })
        })?)
    }
}

#[derive(Debug)]
pub enum WsMessage {
    Ok,
}
