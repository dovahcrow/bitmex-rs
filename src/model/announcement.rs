use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
pub struct GetAnnouncementRequest {
    pub columns: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct GetAnnouncementResponse {
    pub id: Option<usize>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

pub type GetAnnouncementUrgentResponse = GetAnnouncementResponse;
