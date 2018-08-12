use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Announcement {
    id: usize,
    link: String,
    title: String,
    content: String,
    date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    id: String,
    secret: Option<String>, // The document claims this field's existence, but actually not
    name: String,
    nonce: usize,
    cidr: String,
    permissions: Vec<String>,
    enabled: bool,
    user_id: usize,
    created: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    id: usize,
    date: DateTime<Utc>,
    user: String,
    message: String,
    html: String,
    from_bot: bool,
    #[serde(rename = "channelID")]
    channel_id: usize,
}
