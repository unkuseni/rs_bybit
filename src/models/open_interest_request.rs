use crate::prelude::*;

/// Parameters for requesting open interest data.
///
/// This struct defines the parameters for querying open interest via the `/v5/market/open-interest` endpoint. Open interest represents the total number of outstanding contracts, a key metric for assessing market participation and sentiment in perpetual futures.
#[derive(Clone, Default)]
pub struct OpenInterestRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. For perpetual futures, use `Linear` or `Inverse`. Bots must set this to fetch open interest for the correct contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract. Bots must specify a valid symbol to retrieve open interest data.
    pub symbol: Cow<'a, str>,

    /// The time interval for open interest data (e.g., "5min", "1h", "1d").
    ///
    /// Specifies the granularity of the open interest data, such as `5min` (5 minutes), `1h` (1 hour), or `1d` (1 day). Bots should choose an interval that aligns with their analysis timeframe—shorter intervals for intraday strategies, longer for trend analysis.
    pub interval: Cow<'a, str>,

    /// The start time for the open interest data (Unix timestamp in milliseconds).
    ///
    /// Defines the beginning of the time range. Bots should set this for historical open interest analysis, such as studying market participation trends in perpetual futures.
    pub start_time: Option<u64>,

    /// The end time for the open interest data (Unix timestamp in milliseconds).
    ///
    /// Defines the end of the time range. Bots should set this to limit data to a specific period, optimizing performance when processing large datasets.
    pub end_time: Option<u64>,

    /// The maximum number of records to return (1-200, default: 200).
    ///
    /// Controls the number of open interest records returned. Bots should set a reasonable limit to balance data completeness with performance, especially for high-frequency strategies requiring real-time data.
    pub limit: Option<u64>,
}

impl<'a> OpenInterestRequest<'a> {
    /// Creates a default OpenInterest request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, and `interval` set to `"1h"`. Suitable for testing but should be customized for production to match specific trading needs and analysis timeframes.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> OpenInterestRequest<'a> {
        OpenInterestRequest::new(Category::Linear, "BTCUSDT", "1h", None, None, None)
    }
    /// Constructs a new OpenInterest request with specified parameters.
    ///
    /// Allows full customization. Bots should use this to specify the exact symbol, interval, and time range to align with their strategy for analyzing open interest in perpetual futures.
    pub fn new(
        category: Category,
        symbol: &'a str,
        interval: &'a str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> OpenInterestRequest<'a> {
        OpenInterestRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            interval: Cow::Borrowed(interval),
            start_time,
            end_time,
            limit,
        }
    }
}
