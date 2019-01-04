use failure::Error;
use futures::Future;

use crate::error::Result;
use crate::model::user::{
    GetUserAffiliateStatusResponse, GetUserCommissionResponse, GetUserDepositAddressRequest, GetUserDepositAddressResponse, GetUserResponse, GetUserWalletHistoryRequest,
    GetUserWalletHistoryResponse, GetUserWalletRequest, GetUserWalletResponse, GetUserWalletSummaryRequest, GetUserWalletSummaryResponse,
};
use crate::BitMEX;

impl BitMEX {
    pub fn get_user(&self) -> Result<impl Future<Item = GetUserResponse, Error = Error>> {
        Ok(self.transport.signed_get::<_, ()>("/user", None)?)
    }
    pub fn get_user_affiliate_status(&self) -> Result<impl Future<Item = GetUserAffiliateStatusResponse, Error = Error>> {
        Ok(self.transport.signed_get::<_, ()>("/user/affiliateStatus", None)?)
    }

    pub fn get_user_commission(&self) -> Result<impl Future<Item = GetUserCommissionResponse, Error = Error>> {
        Ok(self.transport.signed_get::<_, ()>("/user/commission", None)?)
    }
    pub fn get_user_deposit_address(&self, req: GetUserDepositAddressRequest) -> Result<impl Future<Item = GetUserDepositAddressResponse, Error = Error>> {
        Ok(self.transport.signed_get("/user/depositAddress", Some(req))?)
    }

    pub fn get_user_wallet(&self, req: GetUserWalletRequest) -> Result<impl Future<Item = GetUserWalletResponse, Error = Error>> {
        Ok(self.transport.signed_get("/user/wallet", Some(req))?)
    }
    pub fn get_user_wallet_history(&self, req: GetUserWalletHistoryRequest) -> Result<impl Future<Item = GetUserWalletHistoryResponse, Error = Error>> {
        Ok(self.transport.signed_get("/user/walletHistory", Some(req))?)
    }
    pub fn get_user_wallet_summary(&self, req: GetUserWalletSummaryRequest) -> Result<impl Future<Item = GetUserWalletSummaryResponse, Error = Error>> {
        Ok(self.transport.signed_get("/user/walletSummary", Some(req))?)
    }
}
