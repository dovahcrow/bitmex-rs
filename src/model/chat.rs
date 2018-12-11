use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetChatRequest {
    pub count: usize,
    pub start: Option<usize>,
    pub reverse: bool,
    #[serde(rename = "channelID")]
    pub channel_id: Option<usize>,
}

impl Default for GetChatRequest {
    fn default() -> Self {
        GetChatRequest {
            count: 100,
            reverse: true,
            channel_id: None,
            start: None,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetChatResponse {
    pub id: usize,
    pub date: DateTime<Utc>,
    pub user: String,
    pub message: String,
    pub html: String,
    pub from_bot: bool,
    #[serde(rename = "channelID")]
    pub channel_id: usize,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostChatRequest {
    pub message: String,
    #[serde(rename = "channelID")]
    pub channel_id: usize,
}

impl Default for PostChatRequest {
    fn default() -> Self {
        PostChatRequest {
            message: "".to_string(),
            channel_id: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostChatResponse {
    pub id: usize,
    pub date: DateTime<Utc>,
    pub user: String,
    pub message: String,
    pub html: String,
    pub from_bot: bool,
    #[serde(rename = "channelID")]
    pub channel_id: usize,
}

#[derive(Deserialize, Debug)]
pub struct GetChatChannelsResponse {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatConnectedResponse {
    pub users: usize,
    pub bots: usize,
}
