use crate::prelude::*;

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
    /// Retrieves historical kline (candlestick) data for a trading pair.
    ///
    /// Kline data represents price movements over fixed time intervals and is essential
    /// for technical analysis in trading strategies. This endpoint supports spot,
    /// linear (USDT-margined), and inverse (coin-margined) perpetual contracts.
    ///
    /// # Arguments
    ///
    /// * `req` - A `KlineRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `KlineResponse` if successful, or `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), BybitError> {
    ///     let market = MarketData::new(None, None);
    ///
    ///     // Using builder pattern
    ///     let request = KlineRequest::builder()
    ///         .category(Category::Linear)
    ///         .symbol("BTCUSDT")
    ///         .interval(Interval::H1)
    ///         .limit(100)
    ///         .build()
    ///         .unwrap();
    ///
    ///     let response = market.get_klines(request).await?;
    ///     println!("Retrieved {} klines", response.result.list.len());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `BybitError` if:
    /// - Request parameters are invalid (e.g., limit out of range)
    /// - API returns an error response
    /// - Network or parsing errors occur
    pub async fn get_klines<'b>(&self, req: KlineRequest<'_>) -> Result<KlineResponse, BybitError> {
        // Validate request parameters
        req.validate().map_err(|e| BybitError::Base(e))?;

        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        // Set category (default to Linear if not specified)
        let category = req.category.unwrap_or(Category::Linear);
        parameters.insert("category".to_owned(), category.as_str().to_owned());

        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("interval".into(), req.interval.as_str().to_owned());

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

    /// Retrieves historical mark price kline data for perpetual contracts.
    ///
    /// Mark price is a reference price used to calculate funding rates and trigger
    /// liquidations in perpetual futures contracts. This endpoint supports only
    /// linear (USDT-margined) and inverse (coin-margined) perpetual contracts.
    ///
    /// # Arguments
    ///
    /// * `req` - A `KlineRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `MarkPriceKlineResponse` if successful, or `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), BybitError> {
    ///     let market = MarketData::new(None, None);
    ///
    ///     let request = KlineRequest::builder()
    ///         .category(Category::Linear)
    ///         .symbol("BTCUSDT")
    ///         .interval(Interval::M15)
    ///         .limit(50)
    ///         .build()
    ///         .unwrap();
    ///
    ///     let response = market.get_mark_price_klines(request).await?;
    ///     println!("Retrieved {} mark price klines", response.result.list.len());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `BybitError` if:
    /// - Category is not Linear or Inverse
    /// - Request parameters are invalid
    /// - API returns an error response
    /// - Network or parsing errors occur
    pub async fn get_mark_price_klines<'b>(
        &self,
        req: KlineRequest<'_>,
    ) -> Result<MarkPriceKlineResponse, BybitError> {
        // Validate request parameters
        req.validate().map_err(|e| BybitError::Base(e))?;

        // Validate category (must be Linear or Inverse for mark price klines)
        let category = req.category.unwrap_or(Category::Linear);
        match category {
            Category::Linear | Category::Inverse => {
                // Valid category
            }
            _ => {
                return Err(BybitError::Base(
                    "Category must be either Linear or Inverse for mark price klines".to_string(),
                ))
            }
        }

        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".to_owned(), category.as_str().to_owned());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("interval".into(), req.interval.as_str().to_owned());

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
    /// Retrieves historical index price kline data for perpetual contracts.
    ///
    /// Index price tracks the underlying asset's spot price across multiple exchanges
    /// and is used to anchor the mark price in perpetual futures contracts.
    /// This endpoint supports only linear (USDT-margined) and inverse (coin-margined) perpetual contracts.
    ///
    /// # Arguments
    ///
    /// * `req` - A `KlineRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `IndexPriceKlineResponse` if successful, or `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), BybitError> {
    ///     let market = MarketData::new(None, None);
    ///
    ///     let request = KlineRequest::builder()
    ///         .category(Category::Inverse)
    ///         .symbol("BTCUSD")
    ///         .interval(Interval::H4)
    ///         .limit(200)
    ///         .build()
    ///         .unwrap();
    ///
    ///     let response = market.get_index_price_klines(request).await?;
    ///     println!("Retrieved {} index price klines", response.result.list.len());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `BybitError` if:
    /// - Category is not Linear or Inverse
    /// - Request parameters are invalid
    /// - API returns an error response
    /// - Network or parsing errors occur
    pub async fn get_index_price_klines<'b>(
        &self,
        req: KlineRequest<'_>,
    ) -> Result<IndexPriceKlineResponse, BybitError> {
        // Validate request parameters
        req.validate().map_err(|e| BybitError::Base(e))?;

        // Validate category (must be Linear or Inverse for index price klines)
        let category = req.category.unwrap_or(Category::Linear);
        match category {
            Category::Linear | Category::Inverse => {
                // Valid category
            }
            _ => {
                return Err(BybitError::Base(
                    "Category must be either Linear or Inverse for index price klines".to_string(),
                ))
            }
        }

        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".to_owned(), category.as_str().to_owned());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("interval".into(), req.interval.as_str().to_owned());

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
    /// Retrieves historical premium index price kline data for perpetual contracts.
    ///
    /// Premium index price reflects the premium or discount of the perpetual futures price
    /// relative to the spot index price. This is key for understanding funding rate dynamics.
    /// This endpoint supports only linear (USDT-margined) perpetual contracts.
    ///
    /// # Arguments
    ///
    /// * `req` - A `KlineRequest` containing the query parameters
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `PremiumIndexPriceKlineResponse` if successful, or `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), BybitError> {
    ///     let market = MarketData::new(None, None);
    ///
    ///     let request = KlineRequest::builder()
    ///         .category(Category::Linear)
    ///         .symbol("BTCUSDT")
    ///         .interval(Interval::D1)
    ///         .limit(30)
    ///         .build()
    ///         .unwrap();
    ///
    ///     let response = market.get_premium_index_price_klines(request).await?;
    ///     println!("Retrieved {} premium index klines", response.result.list.len());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `BybitError` if:
    /// - Category is not Linear (premium index only supports linear contracts)
    /// - Request parameters are invalid
    /// - API returns an error response
    /// - Network or parsing errors occur
    pub async fn get_premium_index_price_klines<'b>(
        &self,
        req: KlineRequest<'_>,
    ) -> Result<PremiumIndexPriceKlineResponse, BybitError> {
        // Validate request parameters
        req.validate().map_err(|e| BybitError::Base(e))?;

        // Validate category (must be Linear for premium index klines)
        let category = req.category.unwrap_or(Category::Linear);
        match category {
            Category::Linear => {
                // Valid category
            }
            _ => {
                return Err(BybitError::Base(
                    "Category must be Linear for premium index price klines".to_string(),
                ))
            }
        }

        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".to_owned(), category.as_str().to_owned());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("interval".into(), req.interval.as_str().to_owned());
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
    /// Retrieves a list of instruments (Futures or Spot) based on the specified filters.
    ///
    /// This function queries the exchange for instruments, optionally filtered by the provided
    /// symbol, status, base coin, and result count limit. It supports both Futures and Spot instruments,

    /// returning results encapsulated in the `InstrumentInfo` enum.
    ///
    /// # Arguments
    ///
    /// * `symbol` - An optional filter to specify the symbol of the instruments.
    /// * `status` - An optional boolean to indicate if only instruments with trading status should be retrieved.
    /// * `base_coin` - An optional filter for the base coin of the instruments.
    /// * `limit` - An optional limit on the number of instruments to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result<InstrumentInfoResponse, Error>` where the `Ok` variant contains the filtered list of
    /// instruments (Futures or Spot), and the `Err` variant contains an error if the request fails or if the response
    /// parsing encounters an issue.
    pub async fn get_instrument_info<'b>(
        &self,
        req: InstrumentRequest<'b>,
    ) -> Result<InstrumentInfoResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let category_value = match req.category {
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            Category::Spot => "spot",
            _ => return Err(BybitError::from("Invalid category".to_string())),
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
        let response: InstrumentInfoResponse = self
            .client
            .get(API::Market(Market::InstrumentsInfo), Some(request))
            .await?;
        Ok(response)
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

    /// Asynchronously retrieves RPI (Real-time Price Improvement) order book data.
    ///
    /// This method fetches the RPI order book for a specified trading pair, which includes
    /// both regular orders and RPI orders. RPI orders can provide price improvement for takers
    /// when they cross with non-RPI orders.
    ///
    /// # Arguments
    ///
    /// * `req` - The RPI order book request parameters containing symbol, optional category, and limit.
    ///
    /// # Returns
    ///
    /// A `Result<RPIOrderbookResponse, BybitError>` which is Ok if the RPI order book is successfully retrieved,
    /// or an Err with a detailed error message otherwise.
    pub async fn get_rpi_orderbook<'b>(
        &self,
        req: RPIOrderbookRequest<'_>,
    ) -> Result<RPIOrderbookResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        // Symbol is required
        parameters.insert("symbol".into(), req.symbol.into());

        // Category is optional
        if let Some(category) = req.category {
            parameters.insert("category".into(), category.as_str().into());
        }

        // Limit is required (1-50)
        parameters.insert("limit".to_string(), req.limit.to_string());

        let request = build_request(&parameters);
        let response: RPIOrderbookResponse = self
            .client
            .get(API::Market(Market::RPIOrderbook), Some(request))
            .await?;

        Ok(response)
    }
    /// Asynchronously retrieves tickers based on the provided symbol and category.
    ///
    /// # Arguments
    ///
    /// * `symbol` - An optional reference to a string representing the symbol.
    /// * `category` - The market category (e.g., Linear, Inverse, Spot) for which tickers are to be retrieved.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of Ticker objects, or an error if the retrieval fails.
    pub async fn get_tickers(
        &self,
        symbol: Option<&str>,
        category: Category,
    ) -> Result<TickerResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), category.as_str().into());
        if let Some(symbol) = symbol {
            parameters.insert("symbol".into(), symbol.into());
        }
        let request = build_request(&parameters);
        let response: TickerResponse = self
            .client
            .get(API::Market(Market::Ticker), Some(request))
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
    ) -> Result<OpenInterestResponse, BybitError> {
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
        let response: OpenInterestResponse = self
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

    /// Retrieves new delivery price data for options contracts.
    ///
    /// This method fetches historical option delivery prices from the `/v5/market/new-delivery-price` endpoint.
    /// This endpoint is specifically for options contracts and returns the most recent 50 records
    /// in reverse order of "deliveryTime" by default.
    ///
    /// # Important Notes
    /// - This endpoint only supports options contracts (`category` must be `option`)
    /// - It is recommended to query this endpoint 1 minute after settlement is completed,
    ///   because the data returned by this endpoint may be delayed by 1 minute.
    ///
    /// # Arguments
    ///
    /// * `req` - The new delivery price request parameters containing category, base coin, and optional settle coin.
    ///
    /// # Returns
    ///
    /// A `Result` type containing either a `NewDeliveryPriceSummary` upon success or an error message.
    pub async fn get_new_delivery_price<'b>(
        &self,
        req: NewDeliveryPriceRequest<'_>,
    ) -> Result<NewDeliveryPriceResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        // Category is required and must be "option"
        parameters.insert("category".into(), req.category.as_str().into());

        // Base coin is required
        parameters.insert("baseCoin".into(), req.base_coin.into());

        // Settle coin is optional (defaults to USDT if not specified)
        if let Some(settle_coin) = req.settle_coin {
            parameters.insert("settleCoin".into(), settle_coin.into());
        }

        let request = build_request(&parameters);
        let response: NewDeliveryPriceResponse = self
            .client
            .get(API::Market(Market::NewDeliveryPrice), Some(request))
            .await?;

        Ok(response)
    }

    /// Retrieves ADL (Auto-Deleveraging) alert data.
    ///
    /// This method fetches ADL alert information and insurance pool data from the
    /// `/v5/market/adlAlert` endpoint. ADL is a risk management mechanism that
    /// automatically closes positions when the insurance pool balance reaches
    /// certain thresholds to prevent systemic risk.
    ///
    /// # Important Notes
    /// - Data update frequency is every 1 minute
    /// - Covers: USDT Perpetual, USDT Delivery, USDC Perpetual, USDC Delivery, Inverse Contracts
    /// - The `symbol` parameter is optional; if not provided, returns all symbols
    ///
    /// # Arguments
    ///
    /// * `req` - The ADL alert request parameters containing optional symbol filter.
    ///
    /// # Returns
    ///
    /// A `Result` type containing either an `ADLAlertSummary` upon success or an error message.
    pub async fn get_adl_alert<'b>(
        &self,
        req: ADLAlertRequest<'_>,
    ) -> Result<ADLAlertResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        // Symbol is optional - if provided, filter by symbol
        if let Some(symbol) = req.symbol {
            parameters.insert("symbol".into(), symbol.into());
        }

        let request = build_request(&parameters);
        let response: ADLAlertResponse = self
            .client
            .get(API::Market(Market::ADLAlert), Some(request))
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

    /// Retrieves fee group structure and fee rates.
    ///
    /// This method fetches the fee group structure and fee rates for contract products.
    /// The new grouped fee structure only applies to Pro-level and Market Maker clients
    /// and does not apply to retail traders.
    ///
    /// # Arguments
    ///
    /// * `req` - The fee group info request parameters
    ///
    /// # Returns
    ///
    /// * `Ok(FeeGroupInfoResponse)` - Contains the fee group information
    /// * `Err(BybitError)` - If the request fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_bybit::prelude::*;
    ///
    /// let client = Client::new("api_key", "api_secret");
    /// let market = MarketData::new(client);
    /// let request = FeeGroupInfoRequest::default();
    /// let response = market.get_fee_group_info(request).await?;
    /// ```
    pub async fn get_fee_group_info<'b>(
        &self,
        req: FeeGroupInfoRequest<'_>,
    ) -> Result<FeeGroupInfoResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("productType".into(), req.product_type.into());
        if let Some(group_id) = req.group_id {
            parameters.insert("groupId".into(), group_id.into());
        }
        let request = build_request(&parameters);
        let response: FeeGroupInfoResponse = self
            .client
            .get(API::Market(Market::FeeGroupInfo), Some(request))
            .await?;
        Ok(response)
    }

    /// Retrieves order price limits for a trading symbol.
    ///
    /// This method fetches the highest bid price (buyLmt) and lowest ask price (sellLmt)
    /// for a given symbol, which define the order price limits for derivative or spot trading.
    /// These limits are important for risk management and order validation.
    ///
    /// # Arguments
    ///
    /// * `req` - The order price limit request parameters
    ///
    /// # Returns
    ///
    /// * `Ok(OrderPriceLimitResponse)` - Contains the order price limit information
    /// * `Err(BybitError)` - If the request fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_bybit::prelude::*;
    ///
    /// let client = Client::new("api_key", "api_secret");
    /// let market = MarketData::new(client);
    /// let request = OrderPriceLimitRequest::linear("BTCUSDT");
    /// let response = market.get_order_price_limit(request).await?;
    /// ```
    pub async fn get_order_price_limit<'b>(
        &self,
        req: OrderPriceLimitRequest<'_>,
    ) -> Result<OrderPriceLimitResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(cat) = req.category {
            parameters
                .entry("category".to_owned())
                .or_insert_with(|| cat.as_str().to_owned());
        }
        parameters.insert("symbol".into(), req.symbol.into());
        let request = build_request(&parameters);
        let response: OrderPriceLimitResponse = self
            .client
            .get(API::Market(Market::OrderPriceLimit), Some(request))
            .await?;
        Ok(response)
    }
}
