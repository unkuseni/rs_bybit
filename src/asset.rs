#![allow(unused_imports, unreachable_code, unused_variables)]
use crate::prelude::*;
use serde_json::{json, Value};

use crate::util::{build_json_request, build_request};

#[derive(Clone)]
pub struct AssetManager {
    pub client: Client,
    pub recv_window: u64,
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

    /// Get deposit records (on-chain)
    ///
    /// Query deposit records.
    /// - `endTime` - `startTime` should be less than 30 days. Query last 30 days records by default.
    /// - Support using **main or sub** UID api key to query deposit records respectively.
    ///
    /// # Arguments
    ///
    /// * `req` - A `DepositRecordRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `DepositRecordResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_deposit_records<'a>(
        &self,
        req: DepositRecordRequest<'a>,
    ) -> Result<DepositRecordResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(id) = req.id {
            parameters.insert("id".to_owned(), id.to_string());
        }

        if let Some(tx_id) = req.tx_id {
            parameters.insert("txID".to_owned(), tx_id.to_string());
        }

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
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
        let response: BybitApiResponse<DepositRecordResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryRecord),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get sub deposit records (on-chain)
    ///
    /// Query subaccount's deposit records by **main** UID's API key.
    /// - `endTime` - `startTime` should be less than 30 days. Queries for the last 30 days worth of records by default.
    ///
    /// # Arguments
    ///
    /// * `req` - A `SubDepositRecordRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `SubDepositRecordResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_sub_deposit_records<'a>(
        &self,
        req: SubDepositRecordRequest<'a>,
    ) -> Result<SubDepositRecordResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("subMemberId".to_owned(), req.sub_member_id.to_string());

        if let Some(id) = req.id {
            parameters.insert("id".to_owned(), id.to_string());
        }

        if let Some(tx_id) = req.tx_id {
            parameters.insert("txID".to_owned(), tx_id.to_string());
        }

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
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
        let response: BybitApiResponse<SubDepositRecordResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QuerySubMemberRecord),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get internal deposit records (off-chain)
    ///
    /// Query deposit records within the Bybit platform. These transactions are not on the blockchain.
    /// - The maximum difference between the start time and the end time is 30 days
    /// - Support to get deposit records by Master or Sub Member Api Key
    ///
    /// # Arguments
    ///
    /// * `req` - A `InternalDepositRecordRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `InternalDepositRecordResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_internal_deposit_records<'a>(
        &self,
        req: InternalDepositRecordRequest<'a>,
    ) -> Result<InternalDepositRecordResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(tx_id) = req.tx_id {
            parameters.insert("txID".to_owned(), tx_id.to_string());
        }

        if let Some(start_time) = req.start_time {
            parameters.insert("startTime".to_owned(), start_time.to_string());
        }

        if let Some(end_time) = req.end_time {
            parameters.insert("endTime".to_owned(), end_time.to_string());
        }

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
        }

        if let Some(cursor) = req.cursor {
            parameters.insert("cursor".to_owned(), cursor.to_string());
        }

        if let Some(limit) = req.limit {
            parameters.insert("limit".to_owned(), limit.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<InternalDepositRecordResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryInternalRecord),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get master deposit address
    ///
    /// Query the deposit address information of MASTER account.
    ///
    /// # Arguments
    ///
    /// * `req` - A `MasterDepositAddressRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `MasterDepositAddressResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_master_deposit_address<'a>(
        &self,
        req: MasterDepositAddressRequest<'a>,
    ) -> Result<MasterDepositAddressResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("coin".to_owned(), req.coin.to_string());

        if let Some(chain_type) = req.chain_type {
            parameters.insert("chainType".to_owned(), chain_type.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<MasterDepositAddressResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryAddress),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get sub deposit address
    ///
    /// Query the deposit address information of SUB account.
    /// - Use master UID's api key **only**
    /// - Custodial sub account deposit address cannot be obtained
    ///
    /// # Arguments
    ///
    /// * `req` - A `SubDepositAddressRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `SubDepositAddressResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_sub_deposit_address<'a>(
        &self,
        req: SubDepositAddressRequest<'a>,
    ) -> Result<SubDepositAddressResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("coin".to_owned(), req.coin.to_string());
        parameters.insert("chainType".to_owned(), req.chain_type.to_string());
        parameters.insert("subMemberId".to_owned(), req.sub_member_id.to_string());

        let request = build_request(&parameters);
        let response: BybitApiResponse<SubDepositAddressResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QuerySubmemberAddress),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get withdrawal address list
    ///
    /// Query the withdrawal addresses in the address book.
    ///
    /// # Note
    /// The API key for querying this endpoint must have withdrawal permissions.
    ///
    /// # Arguments
    ///
    /// * `req` - A `WithdrawalAddressRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `WithdrawalAddressResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_withdrawal_address<'a>(
        &self,
        req: WithdrawalAddressRequest<'a>,
    ) -> Result<WithdrawalAddressResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
        }

        if let Some(chain) = req.chain {
            parameters.insert("chain".to_owned(), chain.to_string());
        }

        if let Some(address_type) = req.address_type {
            parameters.insert("addressType".to_owned(), address_type.to_string());
        }

        if let Some(limit) = req.limit {
            parameters.insert("limit".to_owned(), limit.to_string());
        }

        if let Some(cursor) = req.cursor {
            parameters.insert("cursor".to_owned(), cursor.to_string());
        }

        let request = build_request(&parameters);
        let response: BybitApiResponse<WithdrawalAddressResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryWithdrawalAddress),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get withdrawal records
    ///
    /// Query withdrawal records.
    ///
    /// # Notes
    /// - `endTime` - `startTime` should be less than 30 days. Query last 30 days records by default.
    /// - Can query by the master UID's api key **only**
    ///
    /// # Arguments
    ///
    /// * `req` - A `WithdrawalRecordRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `WithdrawalRecordResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_withdrawal_records<'a>(
        &self,
        req: WithdrawalRecordRequest<'a>,
    ) -> Result<WithdrawalRecordResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(withdraw_id) = req.withdraw_id {
            parameters.insert("withdrawID".to_owned(), withdraw_id.to_string());
        }

        if let Some(tx_id) = req.tx_id {
            parameters.insert("txID".to_owned(), tx_id.to_string());
        }

        if let Some(coin) = req.coin {
            parameters.insert("coin".to_owned(), coin.to_string());
        }

        if let Some(withdraw_type) = req.withdraw_type {
            parameters.insert("withdrawType".to_owned(), withdraw_type.to_string());
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
        let response: BybitApiResponse<WithdrawalRecordResponse> = self
            .client
            .get_signed(
                API::Asset(Asset::QueryWithdrawalRecord),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    /// Get available WASPs (Virtual Asset Service Providers)
    ///
    /// This endpoint is used for querying the available WASPs.
    /// This API distinguishes which compliance zone the user belongs to
    /// and the corresponding list of exchanges based on the user's UID.
    ///
    /// # Arguments
    ///
    /// None - This endpoint takes no parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `VaspListResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn get_vasp_list(&self) -> Result<VaspListResponse, BybitError> {
        let response: BybitApiResponse<VaspListResponse> = self
            .client
            .get_signed(API::Asset(Asset::QueryVaspList), self.recv_window, None)
            .await?;
        Ok(response.result)
    }

    /// Create a withdrawal
    ///
    /// Withdraw assets from your Bybit account. You can make an off-chain transfer
    /// if the target wallet address is from Bybit. This means that no blockchain
    /// fee will be charged.
    ///
    /// # Notes
    /// - Although the API rate limit for this endpoint is 5 req/s, there is also a
    ///   secondary limit: you can only withdraw once every 10 seconds per chain/coin combination.
    /// - Make sure you have whitelisted your wallet address
    /// - Request by the master UID's api key **only**
    ///
    /// # Arguments
    ///
    /// * `req` - A `WithdrawRequest` containing the withdrawal parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `WithdrawResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn withdraw<'a>(
        &self,
        req: WithdrawRequest<'a>,
    ) -> Result<WithdrawResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("coin".to_owned(), req.coin.to_string());
        parameters.insert("address".to_owned(), req.address.to_string());
        parameters.insert("amount".to_owned(), req.amount.to_string());
        parameters.insert("timestamp".to_owned(), req.timestamp.to_string());
        parameters.insert("accountType".to_owned(), req.account_type.to_string());

        if let Some(chain) = req.chain {
            parameters.insert("chain".to_owned(), chain.to_string());
        }

        if let Some(tag) = req.tag {
            parameters.insert("tag".to_owned(), tag.to_string());
        }

        if let Some(force_chain) = req.force_chain {
            parameters.insert("forceChain".to_owned(), force_chain.to_string());
        }

        if let Some(fee_type) = req.fee_type {
            parameters.insert("feeType".to_owned(), fee_type.to_string());
        }

        if let Some(request_id) = req.request_id {
            parameters.insert("requestId".to_owned(), request_id.to_string());
        }

        // Handle beneficiary info if present
        if let Some(beneficiary) = req.beneficiary {
            if let Some(beneficiary_transaction_purpose) =
                beneficiary.beneficiary_transaction_purpose
            {
                parameters.insert(
                    "beneficiaryTransactionPurpose".to_owned(),
                    beneficiary_transaction_purpose.to_string(),
                );
            }

            if let Some(beneficiary_representative_first_name) =
                beneficiary.beneficiary_representative_first_name
            {
                parameters.insert(
                    "beneficiaryRepresentativeFirstName".to_owned(),
                    beneficiary_representative_first_name.to_string(),
                );
            }

            if let Some(beneficiary_representative_last_name) =
                beneficiary.beneficiary_representative_last_name
            {
                parameters.insert(
                    "beneficiaryRepresentativeLastName".to_owned(),
                    beneficiary_representative_last_name.to_string(),
                );
            }

            if let Some(vasp_entity_id) = beneficiary.vasp_entity_id {
                parameters.insert("vaspEntityId".to_owned(), vasp_entity_id.to_string());
            }

            if let Some(beneficiary_name) = beneficiary.beneficiary_name {
                parameters.insert("beneficiaryName".to_owned(), beneficiary_name.to_string());
            }

            if let Some(beneficiary_legal_type) = beneficiary.beneficiary_legal_type {
                parameters.insert(
                    "beneficiaryLegalType".to_owned(),
                    beneficiary_legal_type.to_string(),
                );
            }

            if let Some(beneficiary_wallet_type) = beneficiary.beneficiary_wallet_type {
                parameters.insert(
                    "beneficiaryWalletType".to_owned(),
                    beneficiary_wallet_type.to_string(),
                );
            }

            if let Some(beneficiary_unhosted_wallet_type) =
                beneficiary.beneficiary_unhosted_wallet_type
            {
                parameters.insert(
                    "beneficiaryUnhostedWalletType".to_owned(),
                    beneficiary_unhosted_wallet_type.to_string(),
                );
            }

            if let Some(beneficiary_poi_number) = beneficiary.beneficiary_poi_number {
                parameters.insert(
                    "beneficiaryPoiNumber".to_owned(),
                    beneficiary_poi_number.to_string(),
                );
            }

            if let Some(beneficiary_poi_type) = beneficiary.beneficiary_poi_type {
                parameters.insert(
                    "beneficiaryPoiType".to_owned(),
                    beneficiary_poi_type.to_string(),
                );
            }

            if let Some(beneficiary_poi_issuing_country) =
                beneficiary.beneficiary_poi_issuing_country
            {
                parameters.insert(
                    "beneficiaryPoiIssuingCountry".to_owned(),
                    beneficiary_poi_issuing_country.to_string(),
                );
            }

            if let Some(beneficiary_poi_expired_date) = beneficiary.beneficiary_poi_expired_date {
                parameters.insert(
                    "beneficiaryPoiExpiredDate".to_owned(),
                    beneficiary_poi_expired_date.to_string(),
                );
            }

            if let Some(beneficiary_address_country) = beneficiary.beneficiary_address_country {
                parameters.insert(
                    "beneficiaryAddressCountry".to_owned(),
                    beneficiary_address_country.to_string(),
                );
            }

            if let Some(beneficiary_address_state) = beneficiary.beneficiary_address_state {
                parameters.insert(
                    "beneficiaryAddressState".to_owned(),
                    beneficiary_address_state.to_string(),
                );
            }

            if let Some(beneficiary_address_city) = beneficiary.beneficiary_address_city {
                parameters.insert(
                    "beneficiaryAddressCity".to_owned(),
                    beneficiary_address_city.to_string(),
                );
            }

            if let Some(beneficiary_address_building) = beneficiary.beneficiary_address_building {
                parameters.insert(
                    "beneficiaryAddressBuilding".to_owned(),
                    beneficiary_address_building.to_string(),
                );
            }

            if let Some(beneficiary_address_street) = beneficiary.beneficiary_address_street {
                parameters.insert(
                    "beneficiaryAddressStreet".to_owned(),
                    beneficiary_address_street.to_string(),
                );
            }

            if let Some(beneficiary_address_postal_code) =
                beneficiary.beneficiary_address_postal_code
            {
                parameters.insert(
                    "beneficiaryAddressPostalCode".to_owned(),
                    beneficiary_address_postal_code.to_string(),
                );
            }

            if let Some(beneficiary_date_of_birth) = beneficiary.beneficiary_date_of_birth {
                parameters.insert(
                    "beneficiaryDateOfBirth".to_owned(),
                    beneficiary_date_of_birth.to_string(),
                );
            }

            if let Some(beneficiary_place_of_birth) = beneficiary.beneficiary_place_of_birth {
                parameters.insert(
                    "beneficiaryPlaceOfBirth".to_owned(),
                    beneficiary_place_of_birth.to_string(),
                );
            }
        }

        let request = build_json_request(&parameters);
        let response: BybitApiResponse<WithdrawResponse> = self
            .client
            .post_signed(API::Asset(Asset::Withdraw), self.recv_window, Some(request))
            .await?;
        Ok(response.result)
    }

    /// Cancel a withdrawal
    ///
    /// Cancel the withdrawal
    ///
    /// # Arguments
    ///
    /// * `req` - A `CancelWithdrawRequest` containing the withdrawal ID to cancel
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `CancelWithdrawResponse` if successful,
    /// or `BybitError` if an error occurs.
    pub async fn cancel_withdraw<'a>(
        &self,
        req: CancelWithdrawRequest<'a>,
    ) -> Result<CancelWithdrawResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("id".to_owned(), req.id.to_string());

        let request = build_json_request(&parameters);
        let response: BybitApiResponse<CancelWithdrawResponse> = self
            .client
            .post_signed(
                API::Asset(Asset::CancelWithdraw),
                self.recv_window,
                Some(request),
            )
            .await?;
        Ok(response.result)
    }
}
