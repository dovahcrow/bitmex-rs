use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneralApiKeyResponse {
    pub id: String,
    pub secret: Option<String>, // The document claims this field's existence, but actually not
    pub name: String,
    pub nonce: usize,
    pub cidr: String,
    pub permissions: Vec<String>,
    pub enabled: bool,
    pub user_id: usize,
    pub created: DateTime<Utc>,
}

pub type GetApiKeyResponse = GeneralApiKeyResponse;

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostApiKeyRequest {
    pub name: Option<String>,
    pub cidr: Option<String>,
    pub permissions: Option<ApiKeyPermission>,
    pub enabled: bool,
    pub token: Option<String>,
}

pub type PostApiKeyResponse = GeneralApiKeyResponse;

#[derive(Copy, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiKeyPermission {
    Order,
    OrderCancel,
    Withdraw,
}

impl ToString for ApiKeyPermission {
    fn to_string(&self) -> String {
        match self {
            ApiKeyPermission::Order => "order".to_string(),
            ApiKeyPermission::OrderCancel => "orderCancel".to_string(),
            ApiKeyPermission::Withdraw => "withdraw".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Success {
    pub success: bool,
}

#[derive(Serialize, Debug)]
pub struct DeleteApiKeyRequest {
    #[serde(rename = "apiKeyID")]
    pub api_key_id: String,
}
pub type DeleteApiKeyResponse = Success;

pub type PostApiKeyDisableRequest = DeleteApiKeyRequest;
pub type PostApiKeyDisableResponse = GeneralApiKeyResponse;

pub type PostApiKeyEnableRequest = DeleteApiKeyRequest;
pub type PostApiKeyEnableResponse = GeneralApiKeyResponse;
