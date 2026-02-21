#![allow(unused_imports, unreachable_code, unused_variables)]
use crate::prelude::*;
use serde_json::{json, Value};

use crate::util::{build_json_request, build_request};

#[derive(Clone)]
pub struct AssetManager {
    pub client: Client,
    pub recv_window: u16,
}

impl AssetManager {
    /// Get USDC session settlement records
    ///
    /// Query session settlement records of USDC perpetual and futures.
    /// During periods of extreme market volatility, this interface may experience
    /// increased latency or temporary delays in data delivery.
    ///
    /// # Arguments
    ///
    /// * `req` - A `SettlementRecordRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `SettlementRecordResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_settlement_record<'a>(
        &self,
        req: SettlementRecordRequest<'a>,
    ) -> Result<SettlementRecordResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("category".to_owned(), req.category.to_string());

        if let Some(symbol) = req.symbol {
            parameters.insert("symbol".to_owned(), symbol.to_string());
        }

        if let Some(start_time) = req.start_time {
            parameters.insert("startTime".to_owned(), start_time.to_string());
        }

        if let Some(end_time) = req.end_time {
            parameters.insert("endTime".to_owned(), end_time.to_string());
        }

        if let Some(limit) = req.limit {
            parameters.insert("limit".to_owned(), limit.to_string());
        }

        if let Some(cursor) = req.cursor {
            parameters.insert("cursor".to_owned(), cursor.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<SettlementRecordResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::SettlementRecord),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get coin exchange records
    ///
    /// Query the coin exchange records.
    /// It sometimes has 5 secs delay.
    ///
    /// # Arguments
    ///
    /// * `req` - A `CoinExchangeRecordRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `CoinExchangeRecordResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_coin_exchange_records<'a>(
        &self,
        req: CoinExchangeRecordRequest<'a>,
    ) -> Result<CoinExchangeRecordResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(from_coin) = req.from_coin {
            parameters.insert("fromCoin".to_owned(), from_coin.to_string());
        }

        if let Some(to_coin) = req.to_coin {
            parameters.insert("toCoin".to_owned(), to_coin.to_string());
        }

        if let Some(limit) = req.limit {
            parameters.insert("limit".to_owned(), limit.to_string());
        }

        if let Some(cursor) = req.cursor {
            parameters.insert("cursor".to_owned(), cursor.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<CoinExchangeRecordResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::CoinExchangeRecord),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get coin information
    ///
    /// Query coin information, including chain information, withdraw and deposit status.
    ///
    /// # Arguments
    ///
    /// * `req` - A `CoinInfoRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `CoinInfoResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_coin_info<'a>(
        &self,
        req: CoinInfoRequest<'a>,
    ) -> Result<CoinInfoResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<CoinInfoResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryInfo),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get sub UID list
    ///
    /// Query the sub UIDs under a main UID. It returns up to 2000 sub accounts.
    /// Query by the master UID's api key **only**.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `SubUidResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_sub_uid(&self) -> Result<SubUidResponse, BybitError> {
        let response: BybitApiResponse<SubUidResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryTransferSubmemberList),
                self.recv_window,
                None,
            )
            .await?;
        Ok(response.result)
    }

    /// Get delivery record
    ///
    /// Query delivery records of Inverse Futures, USDC Futures, USDT Futures and Options,
    /// sorted by `deliveryTime` in descending order.
    /// During periods of extreme market volatility, this interface may experience
    /// increased latency or temporary delays in data delivery.
    ///
    /// # Arguments
    ///
    /// * `req` - A `DeliveryRecordRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `DeliveryRecordResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_delivery_record<'a>(
        &self,
        req: DeliveryRecordRequest<'a>,
    ) -> Result<DeliveryRecordResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("category".to_owned(), req.category.to_string());

        if let Some(symbol) = req.symbol {
            parameters.insert("symbol".to_owned(), symbol.to_string());
        }

        if let Some(start_time) = req.start_time {
            parameters.insert("startTime".to_owned(), start_time.to_string());
        }

        if let Some(end_time) = req.end_time {
            parameters.insert("endTime".to_owned(), end_time.to_string());
        }

        if let Some(exp_date) = req.exp_date {
            parameters.insert("expDate".to_owned(), exp_date.to_string());
        }

        if let Some(limit) = req.limit {
            parameters.insert("limit".to_owned(), limit.to_string());
        }

        if let Some(cursor) = req.cursor {
            parameters.insert("cursor".to_owned(), cursor.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<DeliveryRecordResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::DeliveryRecord),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get single coin balance
    ///
    /// Query the balance of a specific coin in a specific account type.
    /// Supports querying sub UID's balance. Also, you can check the transferable
    /// amount from master to sub account, sub to master account or sub to sub account,
    /// especially for user who has an institutional loan.
    ///
    /// During periods of extreme market volatility, this interface may experience
    /// increased latency or temporary delays in data delivery.
    ///
    /// # Arguments
    ///
    /// * `req` - A `SingleCoinBalanceRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `SingleCoinBalanceResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_single_coin_balance<'a>(
        &self,
        req: SingleCoinBalanceRequest<'a>,
    ) -> Result<SingleCoinBalanceResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("accountType".to_owned(), req.account_type.to_string());
        parameters.insert("coin".to_owned(), req.coin.to_string());

        if let Some(member_id) = req.member_id {
            parameters.insert("memberId".to_owned(), member_id.to_string());
        }

        if let Some(to_member_id) = req.to_member_id {
            parameters.insert("toMemberId".to_owned(), to_member_id.to_string());
        }

        if let Some(to_account_type) = req.to_account_type {
            parameters.insert("toAccountType".to_owned(), to_account_type.to_string());
        }

        if let Some(with_bonus) = req.with_bonus {
            parameters.insert("withBonus".to_owned(), with_bonus.to_string());
        }

        if let Some(with_transfer_safe_amount) = req.with_transfer_safe_amount {
            parameters.insert(
                "withTransferSafeAmount".to_owned(),
                with_transfer_safe_amount.to_string(),
            );
        }

        if let Some(with_ltv_transfer_safe_amount) = req.with_ltv_transfer_safe_amount {
            parameters.insert(
                "withLtvTransferSafeAmount".to_owned(),
                with_ltv_transfer_safe_amount.to_string(),
            );
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<SingleCoinBalanceResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QuerySingleAccountCoinBalance),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get all coins balance
    ///
    /// You could get all coin balance of all account types under the master account,
    /// and sub account.
    ///
    /// During periods of extreme market volatility, this interface may experience
    /// increased latency or temporary delays in data delivery.
    ///
    /// # Arguments
    ///
    /// * `req` - A `AllCoinsBalanceRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `AllCoinsBalanceResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_all_coins_balance<'a>(
        &self,
        req: AllCoinsBalanceRequest<'a>,
    ) -> Result<AllCoinsBalanceResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("accountType".to_owned(), req.account_type.to_string());

        if let Some(member_id) = req.member_id {
            parameters.insert("memberId".to_owned(), member_id.to_string());
        }

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
        }

        if let Some(with_bonus) = req.with_bonus {
            parameters.insert("withBonus".to_owned(), with_bonus.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<AllCoinsBalanceResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryAccountCoinBalance),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get withdrawable amount
    ///
    /// Query the withdrawable amount for a specific coin.
    ///
    /// During periods of extreme market volatility, this interface may experience
    /// increased latency or temporary delays in data delivery.
    ///
    /// # Arguments
    ///
    /// * `req` - A `WithdrawableAmountRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `WithdrawableAmountResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_withdrawable_amount<'a>(
        &self,
        req: WithdrawableAmountRequest<'a>,
    ) -> Result<WithdrawableAmountResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("coin".to_owned(), req.coin.to_string());

        let request = build_request(&parameters);
        let response: BybitApiResponse<WithdrawableAmountResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::WithdrawableAmount),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Create internal transfer
    ///
    /// Create the internal transfer between different account types under the same UID.
    ///
    /// # Arguments
    ///
    /// * `req` - A `InternalTransferRequest` containing the transfer parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `InternalTransferResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn create_internal_transfer<'a>(
        &self,
        req: InternalTransferRequest<'a>,
    ) -> Result<InternalTransferResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("transferId".to_owned(), req.transfer_id.to_string());
        parameters.insert("coin".to_owned(), req.coin.to_string());
        parameters.insert("amount".to_owned(), req.amount.to_string());
        parameters.insert(
            "fromAccountType".to_owned(),
            req.from_account_type.to_string(),
        );
        parameters.insert("toAccountType".to_owned(), req.to_account_type.to_string());

        let request = build_json_request(&parameters);
        let response: BybitApiResponse<InternalTransferResponse> = self
            .client
            .post_signed(
                API::Asset(Asset::Intertransfer),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get internal transfer records
    ///
    /// Query the internal transfer records between different account types under the same UID.
    ///
    /// # Arguments
    ///
    /// * `req` - A `InternalTransferRecordsRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `InternalTransferRecordsResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_internal_transfer_records<'a>(
        &self,
        req: InternalTransferRecordsRequest<'a>,
    ) -> Result<InternalTransferRecordsResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(transfer_id) = req.transfer_id {
            parameters.insert("transferId".to_owned(), transfer_id.to_string());
        }

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
        }

        if let Some(status) = req.status {
            parameters.insert("status".to_owned(), status.to_string());
        }

        if let Some(start_time) = req.start_time {
            parameters.insert("startTime".to_owned(), start_time.to_string());
        }

        if let Some(end_time) = req.end_time {
            parameters.insert("endTime".to_owned(), end_time.to_string());
        }

        if let Some(limit) = req.limit {
            parameters.insert("limit".to_owned(), limit.to_string());
        }

        if let Some(cursor) = req.cursor {
            parameters.insert("cursor".to_owned(), cursor.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<InternalTransferRecordsResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryTransferList),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Create universal transfer
    ///
    /// Transfer between sub-sub or main-sub.
    ///
    /// # Arguments
    ///
    /// * `req` - A `UniversalTransferRequest` containing the transfer parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `UniversalTransferResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn create_universal_transfer<'a>(
        &self,
        req: UniversalTransferRequest<'a>,
    ) -> Result<UniversalTransferResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("transferId".to_owned(), req.transfer_id.to_string());
        parameters.insert("coin".to_owned(), req.coin.to_string());
        parameters.insert("amount".to_owned(), req.amount.to_string());
        parameters.insert("fromMemberId".to_owned(), req.from_member_id.to_string());
        parameters.insert("toMemberId".to_owned(), req.to_member_id.to_string());
        parameters.insert(
            "fromAccountType".to_owned(),
            req.from_account_type.to_string(),
        );
        parameters.insert("toAccountType".to_owned(), req.to_account_type.to_string());

        let request = build_json_request(&parameters);
        let response: BybitApiResponse<UniversalTransferResponse> = self
            .client
            .post_signed(
                API::Asset(Asset::UniversalTransfer),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get universal transfer records
    ///
    /// Query universal transfer records.
    ///
    /// # Arguments
    ///
    /// * `req` - A `UniversalTransferRecordsRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `UniversalTransferRecordsResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_universal_transfer_records<'a>(
        &self,
        req: UniversalTransferRecordsRequest<'a>,
    ) -> Result<UniversalTransferRecordsResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(transfer_id) = req.transfer_id {
            parameters.insert("transferId".to_owned(), transfer_id.to_string());
        }

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
        }

        if let Some(status) = req.status {
            parameters.insert("status".to_owned(), status.to_string());
        }

        if let Some(start_time) = req.start_time {
            parameters.insert("startTime".to_owned(), start_time.to_string());
        }

        if let Some(end_time) = req.end_time {
            parameters.insert("endTime".to_owned(), end_time.to_string());
        }

        if let Some(limit) = req.limit {
            parameters.insert("limit".to_owned(), limit.to_string());
        }

        if let Some(cursor) = req.cursor {
            parameters.insert("cursor".to_owned(), cursor.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<UniversalTransferRecordsResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryUniversalTransferList),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get transferable coin list
    ///
    /// Query the transferable coin list between each account type.
    ///
    /// # Arguments
    ///
    /// * `req` - A `TransferableCoinRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `TransferableCoinResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_transferable_coin_list<'a>(
        &self,
        req: TransferableCoinRequest<'a>,
    ) -> Result<TransferableCoinResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert(
            "fromAccountType".to_owned(),
            req.from_account_type.to_string(),
        );
        parameters.insert("toAccountType".to_owned(), req.to_account_type.to_string());

        let request = build_request(&parameters);
        let response: BybitApiResponse<TransferableCoinResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryTransferCoinList),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Set deposit account
    ///
    /// Set auto transfer account after deposit. The same function as the setting
    /// for Deposit on web GUI.
    ///
    /// Only **main** UID can access.
    ///
    /// # Arguments
    ///
    /// * `req` - A `SetDepositAccountRequest` containing the account type
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `SetDepositAccountResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn set_deposit_account<'a>(
        &self,
        req: SetDepositAccountRequest<'a>,
    ) -> Result<SetDepositAccountResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("accountType".to_owned(), req.account_type.to_string());

        let request = build_json_request(&parameters);
        let response: BybitApiResponse<SetDepositAccountResponse> = self
            .client
            .post_signed(
                API::Asset(Asset::SetDepositAccount),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }
}
