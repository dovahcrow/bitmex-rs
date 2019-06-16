use super::definitions::UserEvent;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUserEventRequest {
    pub count: i64,
    pub start_id: i64,
}

impl Default for GetUserEventRequest {
    fn default() -> Self {
        Self {
            count: 150,
            start_id: 1,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUserEventResponse {
    pub user_events: Vec<UserEvent>,
}
