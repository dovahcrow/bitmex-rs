use super::definitions::{Transaction, User, UserCommission, Wallet};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type GetUserResponse = User;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserWalletRequest {
    pub currency: Option<String>, // default is XBt
}

pub type GetUserWalletResponse = Wallet;

#[derive(Clone, Debug, Default, Serialize)]
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

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDepositAddressRequest {
    pub currency: Option<String>, // default is XBt
}

pub type GetUserDepositAddressResponse = String;

/// The struct from swagger is broken, use this one
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserAffiliateStatusResponse {
    pub account: Option<i64>,
    pub currency: Option<String>,
    pub prev_payout: Option<i64>,
    pub prev_turnover: Option<i64>,
    pub prev_comm: Option<i64>,
    pub prev_timestamp: Option<DateTime<Utc>>,
    pub exec_turnover: Option<i64>,
    pub exec_comm: Option<i64>,
    pub total_referrals: Option<i64>,
    pub total_turnover: Option<i64>,
    pub total_comm: Option<i64>,
    pub payout_pcnt: Option<f64>,
    pub pending_payout: Option<i64>,
    pub timestamp: Option<DateTime<Utc>>,
    pub referrer_account: Option<f64>,
    pub referral_discount: Option<f64>,
    pub affiliate_payout: Option<f64>,
}
