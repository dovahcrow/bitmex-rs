use super::definitions::{Transaction, User, UserCommission, Wallet};
use std::collections::BTreeMap;

pub type GetUserResponse = User;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserWalletRequest {
    pub currency: Option<String>, // default is XBt
}

pub type GetUserWalletResponse = Wallet;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserWalletHistoryRequest {
    pub currency: Option<String>, // default is XBt
}

pub type GetUserWalletHistoryResponse = Vec<Transaction>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserWalletSummaryRequest {
    pub currency: Option<String>, // default is XBt
}

pub type GetUserWalletSummaryResponse = Vec<Wallet>;

pub type GetUserCommissionResponse = BTreeMap<String, UserCommission>;
