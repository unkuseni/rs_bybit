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
}
