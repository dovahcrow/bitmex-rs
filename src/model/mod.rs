pub mod announcement;
pub mod api_key;
pub mod chat;
pub mod execution;
pub mod funding;
pub mod global_notification;
pub mod instrument;
pub mod order;
pub mod websocket;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Side {
    Buy,
    Sell,
}
