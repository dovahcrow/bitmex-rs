use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGlobalNotificationResponse {
    pub id: usize,
    pub date: DateTime<Utc>,
    pub title: String,
    pub body: String,
    pub ttl: usize,
    // type: success,
    pub closable: bool,
    pub persist: bool,
    pub wait_for_visibility: bool,
    pub sound: String,
}
