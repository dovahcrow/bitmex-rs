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
    currency: Option<String>, // default is XBt
}

pub type GetUserDepositAddressResponse = String;

/// The struct from swagger is broken, use this one
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserAffiliateStatusResponse {
    account: Option<i64>,
    currency: Option<String>,
    prev_payout: Option<i64>,
    prev_turnover: Option<i64>,
    prev_comm: Option<i64>,
    prev_timestamp: Option<DateTime<Utc>>,
    exec_turnover: Option<i64>,
    exec_comm: Option<i64>,
    total_referrals: Option<i64>,
    total_turnover: Option<i64>,
    total_comm: Option<i64>,
    payout_pcnt: Option<f64>,
    pending_payout: Option<i64>,
    timestamp: Option<DateTime<Utc>>,
    referrer_account: Option<f64>,
    referral_discount: Option<f64>,
    affiliate_payout: Option<f64>,
}
