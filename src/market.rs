use crate::api::{Market, API};
use crate::client::Client;
use crate::errors::BybitError;
use crate::model::{
    Category, DeliveryPriceResponse, FundingHistoryRequest, FundingRateResponse,
    FuturesInstrumentsInfoResponse, FuturesTickersResponse, HistoricalVolatilityRequest,
    HistoricalVolatilityResponse, IndexPriceKlineResponse, InstrumentRequest, InsuranceResponse,
    KlineRequest, KlineResponse, LongShortRatioResponse, MarkPriceKlineResponse,
    OpenInterestRequest, OpeninterestResponse, OptionsInstrument, OrderBookResponse,
    OrderbookRequest, PremiumIndexPriceKlineResponse, RecentTradesRequest, RecentTradesResponse,
    RiskLimitRequest, RiskLimitResponse, SpotInstrumentsInfoResponse, SpotTickersResponse,
};
use crate::util::{build_request, date_to_milliseconds};

use std::collections::BTreeMap;

#[derive(Clone)]
pub struct MarketData {
    pub client: Client,
    pub recv_window: u16,
}

/// Market Data endpoints

impl MarketData {
    /// Retrieves historical price klines.
    ///
    /// This method fetches historical klines (candlestick data) for a specified category, trading pair,
    /// and interval. It supports additional parameters to define a date range and to limit the response size.
    ///
    /// Suitable for USDT perpetual, USDC contract, and Inverse contract categories.
    ///
    /// # Arguments
    ///
    /// * `category` - The market category for which to retrieve klines (optional).
    /// * `symbol` - The trading pair or symbol for which to retrieve klines.
    /// * `interval` - The time interval between klines.
    /// * `start` - The start date for the kline data retrieval in `DDMMYY` format (optional).
    /// * `end` - The end date for the kline data retrieval in `DDMMYY` format (optional).
    /// * `limit` - The maximum number of klines to return (optional).
    ///
    /// # Returns
    ///
    /// A `Result<Vec<KlineData>, Error>` containing the requested kline data if successful, or an error otherwise.
    pub async fn get_klines<'b>(&self, req: KlineRequest<'_>) -> Result<KlineResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(cat) = req.category {
            parameters
                .entry("category".to_owned())
                .or_insert_with(|| cat.as_str().to_owned());
        } else {
            parameters
                .entry("category".to_owned())
                .or_insert_with(|| Category::Linear.as_str().to_owned());
        }
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("interval".into(), req.interval.into());
        if let Some(start_str) = req.start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("start".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = req.end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }
        if let Some(l) = req.limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: KlineResponse = self
            .client
            .get(API::Market(Market::Kline), Some(request))
            .await?;
        Ok(response)
    }
    /// Retrieves historical mark price klines.
    ///
    /// Provides historical kline data for mark prices based on the specified category, symbol, and interval.
    /// Optional parameters can be used to define the range of the data with start and end times, as well as
    /// to limit the number of kline entries returned. This function supports queries for USDT perpetual,
    /// USDC contract, and Inverse contract categories.
    ///
    /// # Arguments
    ///
    /// * `category` - An optional category of the contract, if specified.
    /// * `symbol` - The trading pair or contract symbol.
    /// * `interval` - The interval between klines (e.g., "5m" for five minutes).
    /// * `start` - An optional start time for filtering the data, formatted as "DDMMYY".
    /// * `end` - An optional end time for filtering the data, formatted as "DDMMYY".
    /// * `limit` - An optional limit to the number of kline entries to be returned.
    ///
    /// # Returns
    ///
    /// A `Result<Vec<MarkPriceKline>, Error>` containing the historical mark price kline data if successful,
    /// or an error otherwise.

    pub async fn get_mark_price_klines<'b>(
        &self,
        req: KlineRequest<'_>,
    ) -> Result<MarkPriceKlineResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(category) = req.category {
            match category {
                Category::Linear | Category::Inverse => {
                    parameters.insert("category".to_owned(), category.as_str().to_owned());
                }
                _ => {
                    return Err(BybitError::from(
                        "Category must be either Linear or Inverse".to_string(),
                    ))
                }
            }
        } else {
            parameters.insert("category".to_owned(), Category::Linear.as_str().to_string());
        }
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("interval".into(), req.interval.into());
        if let Some(start_str) = req.start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("start".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = req.end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }

        if let Some(l) = req.limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: MarkPriceKlineResponse = self
            .client
            .get(API::Market(Market::MarkPriceKline), Some(request))
            .await?;
        Ok(response)
    }
    /// Fetches index price klines based on specified criteria.
    ///
    /// Retrieves klines (candlestick data) for index prices given a category, symbol, interval, and optional date range.
    /// The `start` and `end` parameters can define a specific time range for the data, and `limit` controls the number
    /// of klines returned. If `start`, `end`, or `limit` are `None`, they are omitted from the query.
    ///
    /// # Arguments
    ///
    /// * `category` - An optional `Category` determining the contract category.
    /// * `symbol` - The trading pair or symbol for the klines.
    /// * `interval` - The duration between individual klines.
    /// * `start` - Optional start time for the kline data as a string slice.
    /// * `end` - Optional end time for the kline data as a string slice.
    /// * `limit` - Optional maximum number of klines to return.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Vec<Kline>, Error>` with the kline data if the query is successful, or an error detailing
    /// the problem if the query fails.
    pub async fn get_index_price_klines<'b>(
        &self,
        req: KlineRequest<'_>,
    ) -> Result<IndexPriceKlineResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(category) = req.category {
            match category {
                Category::Linear | Category::Inverse => {
                    parameters.insert("category".to_owned(), category.as_str().to_owned());
                }
                _ => {
                    return Err(BybitError::from(
                        "Category must be either Linear or Inverse".to_string(),
                    ))
                }
            }
        } else {
            parameters.insert("category".to_owned(), Category::Linear.as_str().to_string());
        }
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("interval".into(), req.interval.into());
        if let Some(start_str) = req.start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("start".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = req.end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }

        if let Some(l) = req.limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: IndexPriceKlineResponse = self
            .client
            .get(API::Market(Market::IndexPriceKline), Some(request))
            .await?;
        Ok(response)
    }
    /// Retrieves premium index price klines based on specified criteria.
    ///
    /// Given a `symbol` and an `interval`, this function fetches the premium index price klines. It also
    /// accepts optional parameters `start` and `end` to define a specific time range, and `limit` to
    /// restrict the number of klines returned. If `start`, `end`, or `limit` are `None`, they will be
    /// excluded from the query.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The trading pair or symbol for which the klines are to be retrieved.
    /// * `interval` - The duration between individual klines.
    /// * `start` - Optional start time for the kline data.
    /// * `end` - Optional end time for the kline data.
    /// * `limit` - Optional maximum number of klines to be returned.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping a `PremiumIndexPriceKlineSummary` if the query succeeds, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, if there is an issue parsing the response, or if an error
    /// is returned from the server.
    pub async fn get_premium_index_price_klines<'b>(
        &self,
        req: KlineRequest<'_>,
    ) -> Result<PremiumIndexPriceKlineResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".to_owned(), Category::Linear.as_str().to_string());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("interval".into(), req.interval.into());
        if let Some(start_str) = req.start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("start".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = req.end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }
        if let Some(l) = req.limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: PremiumIndexPriceKlineResponse = self
            .client
            .get(API::Market(Market::PremiumIndexPriceKline), Some(request))
            .await?;
        Ok(response)
    }
    /// Retrieves a list of futures instruments based on the specified filters.
    ///
    /// This function queries the exchange for futures instruments, optionally filtered by the provided
    /// symbol, status, base coin, and result count limit.
    ///
    /// # Arguments
    ///
    /// * `symbol` - An optional filter to specify the symbol of the futures instruments.
    /// * `status` - An optional boolean to indicate if only instruments with trading status should be retrieved.
    /// * `base_coin` - An optional filter for the base coin of the futures instruments.
    /// * `limit` - An optional limit on the number of futures instruments to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result<Vec<FuturesInstrument>, Error>` where the `Ok` variant contains the filtered list of
    /// futures instruments, and the `Err` variant contains an error if the request fails or if the response
    /// parsing encounters an issue.
    pub async fn get_futures_instrument_info<'b>(
        &self,
        req: InstrumentRequest<'b>,
    ) -> Result<FuturesInstrumentsInfoResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let category_value = match req.category {
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            _ => {
                return Err(BybitError::from(
                    "Category must be either Linear or Inverse".to_string(),
                ))
            }
        };
        parameters.insert("category".into(), category_value.into());
        if let Some(symbol) = req.symbol {
            parameters.insert("symbol".into(), symbol.into());
        }
        if req.status.unwrap_or(false) {
            parameters.insert("status".into(), "Trading".into());
        }
        if let Some(base_coin) = req.base_coin {
            parameters.insert("baseCoin".into(), base_coin.into());
        }
        if let Some(l) = req.limit {
            parameters.insert("limit".into(), l.to_string());
        }
        let request = build_request(&parameters);
        let response: FuturesInstrumentsInfoResponse = self
            .client
            .get(API::Market(Market::InstrumentsInfo), Some(request))
            .await?;
        Ok(response)
    }

    /// Fetches details for spot instruments based on provided filters.
    ///
    /// # Arguments
    ///
    /// * `symbol` - An optional string to filter instruments by their symbol.
    /// * `status` - A boolean to include only instruments that are currently trading when set to `true`.
    /// * `base_coin` - An optional string to filter instruments by their base coin.
    /// * `limit` - An optional usize to limit the number of instruments returned.
    ///
    /// # Returns
    ///
    /// A `Result` containing a list of `SpotInstrument` instances matching the filters, or an error on failure.
    ///
    pub async fn get_spot_instrument_info<'b>(
        &self,
        req: InstrumentRequest<'_>,
    ) -> Result<SpotInstrumentsInfoResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), "Spot".into());
        if let Some(symbol) = req.symbol {
            parameters.insert("symbol".into(), symbol.into());
        }
        if let Some(status) = req.status {
            if status {
                parameters.insert("status".into(), "Trading".into());
            }
        }
        if let Some(base_coin) = req.base_coin {
            parameters.insert("baseCoin".into(), base_coin.into());
        }
        if let Some(l) = req.limit {
            parameters.insert("limit".into(), l.to_string());
        }
        let request = build_request(&parameters);
        let response: SpotInstrumentsInfoResponse = self
            .client
            .get(API::Market(Market::InstrumentsInfo), Some(request))
            .await?;
        Ok(response)
    }

    pub async fn get_options_instrument_info<'b>(
        &self,
        _req: InstrumentRequest<'_>,
    ) -> Result<Vec<OptionsInstrument>, BybitError> {
        todo!()
    }

    /// Asynchronously fetches the order book depth for a specified symbol within a certain category.
    /// Optionally, the number of order book entries returned can be limited.
    ///
    /// # Arguments
    ///
    /// * `req` - An `OrderbookRequest` containing:
    ///     * `symbol`: The symbol string to query the order book for.
    ///     * `category`: The market category to filter the order book by.
    ///     * `limit`: An optional usize to restrict the number of entries in the order book.
    ///
    /// # Returns
    ///
    /// A `Result<OrderBook, Error>` which is Ok if the order book is successfully retrieved,
    /// or an Err with a detailed error message otherwise.
    pub async fn get_depth<'b>(
        &self,
        req: OrderbookRequest<'_>,
    ) -> Result<OrderBookResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        if let Some(l) = req.limit {
            parameters.insert("limit".to_string(), l.to_string());
        }
        let request = build_request(&parameters);
        let response: OrderBookResponse = self
            .client
            .get(API::Market(Market::OrderBook), Some(request))
            .await?;

        Ok(response)
    }

    /// Asynchronously retrieves spot tickers based on the provided symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - An optional reference to a string representing the symbol.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of SpotTicker objects, or an error if the retrieval fails.
    pub async fn get_spot_tickers(
        &self,
        symbol: Option<&str>,
    ) -> Result<SpotTickersResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), Category::Spot.as_str().into());
        if let Some(symbol) = symbol {
            parameters.insert("symbol".into(), symbol.into());
        }
        let request = build_request(&parameters);
        let response: SpotTickersResponse = self
            .client
            .get(API::Market(Market::Tickers), Some(request))
            .await?;
        Ok(response)
    }

    /// Asynchronously retrieves Futures tickers based on the provided symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - An optional reference to a string representing the symbol.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of FuturesTicker objects, or an error if the retrieval fails.
    pub async fn get_futures_tickers(
        &self,
        symbol: Option<&str>,
    ) -> Result<FuturesTickersResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), Category::Linear.as_str().into());
        if let Some(symbol) = symbol {
            parameters.insert("symbol".into(), symbol.into());
        }
        let request = build_request(&parameters);
        let response: FuturesTickersResponse = self
            .client
            .get(API::Market(Market::Tickers), Some(request))
            .await?;
        Ok(response)
    }

    /// Asynchronously retrieves the funding history based on specified criteria.
    ///
    /// This function obtains historical funding rates for futures contracts given a category,
    /// symbol, and an optional time range and limit. Only Linear or Inverse categories are supported.
    ///
    /// # Arguments
    ///
    /// * `category` - Specifies the contract category (Linear or Inverse).
    /// * `symbol` - The trading pair or contract symbol.
    /// * `start` - An optional parameter indicating the start time for the funding history.
    /// * `end` - An optional parameter indicating the end time for the funding history.
    /// * `limit` - An optional parameter specifying the maximum number of funding rates to return.
    ///
    /// # Returns
    ///
    /// A `Result<Vec<FundingRate>, Error>` representing the historical funding rates if the request is successful,
    /// otherwise an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified category is invalid or if there is a failure during the API request.
    pub async fn get_funding_history<'b>(
        &self,
        req: FundingHistoryRequest<'_>,
    ) -> Result<FundingRateResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let category_value = match req.category {
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            _ => {
                return Err(BybitError::from(
                    "Category must be either Linear or Inverse".to_string(),
                ))
            }
        };
        parameters.insert("category".into(), category_value.into());
        parameters.insert("symbol".into(), req.symbol.into());

        if let Some(start_time) = req.start_time {
            parameters
                .entry("startTime".to_owned())
                .or_insert_with(|| start_time.to_string());
        }

        if let Some(end_time) = req.end_time {
            parameters
                .entry("endTime".to_owned())
                .or_insert_with(|| end_time.to_string());
        }

        if let Some(l) = req.limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: FundingRateResponse = self
            .client
            .get(API::Market(Market::FundingRate), Some(request))
            .await?;
        Ok(response)
    }
    /// Retrieves a list of the most recent trades for a specified market category.
    /// Filtering by symbol and basecoin is supported, and the number of trades returned can be limited.
    ///
    /// # Parameters
    ///
    /// * `category`: The market category to filter trades.
    /// * `symbol`: A specific symbol to filter trades (optional).
    /// * `basecoin`: A specific basecoin to filter trades (optional).
    /// * `limit`: The maximum number of trades to return (optional).
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Trade>)` containing the recent trades if the operation is successful,
    /// or an `Err` with an error message if it fails.
    pub async fn get_recent_trades<'b>(
        &self,
        req: RecentTradesRequest<'_>,
    ) -> Result<RecentTradesResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        if let Some(s) = req.symbol {
            parameters.insert("symbol".into(), s.into());
        }
        if let Some(b) = req.base_coin {
            parameters.insert("baseCoin".into(), b.into());
        }
        if let Some(l) = req.limit {
            parameters.insert("limit".into(), l.to_string());
        }
        let request = build_request(&parameters);
        let response: RecentTradesResponse = self
            .client
            .get(API::Market(Market::RecentTrades), Some(request))
            .await?;

        Ok(response)
    }

    /// Retrieves open interest for a specific market category and symbol over a defined time interval.
    ///
    /// Open interest is the total number of outstanding derivative contracts, such as futures or options,
    /// that have not been settled. This function provides a summary of such open interests.
    ///
    /// # Arguments
    ///
    /// * `category`: The market category to query for open interest data.
    /// * `symbol`: The trading symbol for which open interest is to be retrieved.
    /// * `interval_time`: The duration over which open interest data should be aggregated.
    /// * `start`: The starting point of the time interval (optional).
    /// * `end`: The endpoint of the time interval (optional).
    /// * `limit`: A cap on the number of data points to return (optional).
    ///
    /// # Returns
    ///
    /// A `Result<OpenInterestSummary, Error>` representing either:
    /// - An `OpenInterestSummary` on success, encapsulating the open interest data.
    /// - An `Error`, if the retrieval fails.
    pub async fn get_open_interest<'b>(
        &self,
        req: OpenInterestRequest<'_>,
    ) -> Result<OpeninterestResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let category_value = match req.category {
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            _ => {
                return Err(BybitError::from(
                    "Category must be either Linear or Inverse".to_string(),
                ))
            }
        };
        parameters.insert("category".into(), category_value.into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("intervalTime".into(), req.interval.into());
        if let Some(start_str) = req.start.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("startTime".to_owned())
                .or_insert_with(|| start_millis.to_string());
        }
        if let Some(end_str) = req.end.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("endTime".to_owned())
                .or_insert_with(|| end_millis.to_string());
        }
        if let Some(l) = req.limit {
            parameters
                .entry("limit".to_owned())
                .or_insert_with(|| l.to_string());
        }
        let request = build_request(&parameters);
        let response: OpeninterestResponse = self
            .client
            .get(API::Market(Market::OpenInterest), Some(request))
            .await?;
        Ok(response)
    }
    /// Fetches historical volatility data for a specified base coin.
    ///
    /// This function queries historical volatility based on the given base coin and optional
    /// parameters for the period, start, and end times to filter the results.
    ///
    /// # Arguments
    ///
    /// * `base_coin` - The base coin identifier for which volatility data is being requested.
    /// * `period` - (Optional) A string specifying the period over which to calculate volatility.
    /// * `start` - (Optional) A string indicating the start time for the data range.
    /// * `end` - (Optional) A string indicating the end time for the data range.
    ///
    /// # Returns
    ///
    /// A `Result<Vec<HistoricalVolatility>, Error>` which is either:
    /// - A vector of `HistoricalVolatility` instances within the specified time range on success.
    /// - An `Error` if the request fails or if invalid arguments are provided.
    pub async fn get_historical_volatility<'b>(
        &self,
        req: HistoricalVolatilityRequest<'_>,
    ) -> Result<HistoricalVolatilityResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), Category::Option.as_str().into());
        if let Some(b) = req.base_coin {
            parameters.insert("baseCoin".into(), b.into());
        }
        if let Some(p) = req.period {
            parameters.insert("period".into(), p.into());
        }
        if let Some(s) = req.start {
            let start_millis = date_to_milliseconds(s.as_ref());
            parameters.insert("startTime".into(), start_millis.to_string());
        }
        if let Some(e) = req.end {
            let end_millis = date_to_milliseconds(e.as_ref());
            parameters.insert("endTime".into(), end_millis.to_string());
        }
        let request = build_request(&parameters);
        let response: HistoricalVolatilityResponse = self
            .client
            .get(API::Market(Market::HistoricalVolatility), Some(request))
            .await?;
        Ok(response)
    }

    /// Fetches insurance information for a specific coin.
    ///
    /// # Arguments
    ///
    /// * `coin` - An optional parameter representing the coin for which insurance information is to be fetched.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the insurance summary if successful, or an error if not.
    pub async fn get_insurance(&self, coin: Option<&str>) -> Result<InsuranceResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), Category::Option.as_str().into());
        if let Some(c) = coin {
            parameters.insert("coin".into(), c.into());
        }
        let request = build_request(&parameters);
        let response: InsuranceResponse = self
            .client
            .get(API::Market(Market::Insurance), Some(request))
            .await?;
        Ok(response)
    }

    /// Retrieves the risk limit information based on market category and specific symbol if provided.
    ///
    /// # Parameters
    ///
    /// * `category` - Market category to query for risk limits.
    /// * `symbol` - Optional symbol to further filter the risk limit results.
    ///
    /// # Returns
    ///
    /// A `Result<RiskLimitSummary>` which is either the risk limit details on success or an error on failure.
    pub async fn get_risk_limit<'b>(
        &self,
        req: RiskLimitRequest<'_>,
    ) -> Result<RiskLimitResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let category_value = match req.category {
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            _ => {
                return Err(BybitError::from(
                    "Category must be either Linear or Inverse".to_string(),
                ))
            }
        };
        parameters.insert("category".into(), category_value.into());
        if let Some(s) = req.symbol {
            parameters.insert("symbol".into(), s.into());
        }
        let request = build_request(&parameters);
        let response: RiskLimitResponse = self
            .client
            .get(API::Market(Market::RiskLimit), Some(request))
            .await?;
        Ok(response)
    }

    /// Retrieves the delivery price for a given category, symbol, base coin, and limit.
    ///
    /// # Arguments
    ///
    /// * `category` - The market category to fetch the delivery price from.
    /// * `symbol` - Optional symbol filter for the delivery price.
    /// * `base_coin` - Optional base coin filter for the delivery price.
    /// * `limit` - Optional limit for the delivery price.
    ///
    /// # Returns
    ///
    /// A `Result` type containing either a `DeliveryPriceSummary` upon success or an error message.
    pub async fn get_delivery_price(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        limit: Option<u64>,
    ) -> Result<DeliveryPriceResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), category.as_str().into());
        if let Some(s) = symbol {
            parameters.insert("symbol".into(), s.into());
        }
        if let Some(b) = base_coin {
            parameters.insert("baseCoin".into(), b.into());
        }
        if let Some(l) = limit {
            parameters.insert("limit".into(), l.to_string());
        }
        let request = build_request(&parameters);
        let response: DeliveryPriceResponse = self
            .client
            .get(API::Market(Market::DeliveryPrice), Some(request))
            .await?;
        Ok(response)
    }

    /// Retrieves the long/short ratio for a given market category, symbol, period, and limit.
    ///
    /// The long/short ratio represents the total long position volume divided by the total
    /// short position volume, aggregated from all users. This can provide insight into market
    /// sentiment for a given trading pair during the specified time period.
    ///
    /// # Arguments
    ///
    /// * `category` - The market category (Linear or Inverse) to fetch the long/short ratio from.
    /// * `symbol` - The trading symbol to fetch the long/short ratio for.
    /// * `period` - The period for which to fetch the ratio (e.g., "5min", "15min", "1h").
    /// * `limit` - Optional limit for the number of data points to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` type containing either a `LongShortRatioSummary` upon success or an error message.

    pub async fn get_longshort_ratio(
        &self,
        category: Category,
        symbol: &str,
        period: &str,
        limit: Option<u64>,
    ) -> Result<LongShortRatioResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        match category {
            Category::Linear | Category::Inverse => {
                parameters.insert("category".into(), category.as_str().into())
            }
            _ => {
                return Err(BybitError::from(
                    "Category must be either Linear or Inverse".to_string(),
                ))
            }
        };
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("period".into(), period.into());
        if let Some(l) = limit {
            parameters.insert("limit".into(), l.to_string());
        }
        let request = build_request(&parameters);
        let response: LongShortRatioResponse = self
            .client
            .get(API::Market(Market::LongShortRatio), Some(request))
            .await?;
        Ok(response)
    }
}
