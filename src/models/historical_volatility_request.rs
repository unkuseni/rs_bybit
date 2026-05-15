use crate::prelude::*;

/// Represents a request to fetch historical volatility data for a specific cryptocurrency on Bybit.
/// Historical volatility measures the price fluctuation of an asset over a period, crucial for
/// assessing risk in perpetual futures trading. In perpetual futures, which are contracts without
/// an expiration date, volatility data helps traders predict price movements and adjust leverage.
/// Trading bots use this to optimize entry/exit points or hedge positions.
#[derive(Clone, Default)]
pub struct HistoricalVolatilityRequest<'a> {
    /// The base cryptocurrency (e.g., "BTC" for Bitcoin).
    /// On Bybit, this specifies the asset for which volatility is calculated. For perpetual futures,

    /// the base coin determines the trading pair (e.g., BTCUSD). Bots must validate this field to
    /// ensure the pair is supported, as an invalid coin will result in an API error.
    pub base_coin: Option<Cow<'a, str>>,

    /// The time period for volatility calculation (e.g., "7" for 7 days).
    /// This defines the lookback period for the volatility metric, typically in days. In perpetual
    /// futures, shorter periods (e.g., 7 days) are used for short-term trading strategies, while
    /// longer periods (e.g., 30 days) suit risk management. Bots should handle this flexibly to
    /// adapt to different trading horizons.
    pub period: Option<Cow<'a, str>>,

    /// The start time for the data range (e.g., "2023-01-01T00:00:00Z").
    /// Specifies the beginning of the historical data window. In perpetual futures, accurate
    /// time ranges are critical for backtesting strategies. Bots must ensure the format complies
    /// with Bybit’s API (ISO 8601) to avoid errors.
    pub start: Option<Cow<'a, str>>,

    /// The end time for the data range (e.g., "2023-12-31T23:59:59Z").
    /// Marks the end of the historical data window. This is essential for defining precise data
    /// sets in trading algorithms. Bots should validate that `end` is later than `start` to prevent
    /// invalid requests.
    pub end: Option<Cow<'a, str>>,
}

impl<'a> HistoricalVolatilityRequest<'a> {
    /// Creates a default request with BTC as the base coin.
    /// Useful for quick initialization in trading bots, but developers should override fields as
    /// needed for specific strategies.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> HistoricalVolatilityRequest<'a> {
        HistoricalVolatilityRequest::new(Some("BTC"), None, None, None)
    }
    /// Constructs a new request with specified parameters.
    /// Allows fine-grained control over the volatility query, enabling bots to target specific
    /// coins and timeframes.
    pub fn new(
        base_coin: Option<&'a str>,
        period: Option<&'a str>,
        start: Option<&'a str>,
        end: Option<&'a str>,
    ) -> HistoricalVolatilityRequest<'a> {
        HistoricalVolatilityRequest {
            base_coin: base_coin.map(Cow::Borrowed),
            period: period.map(Cow::Borrowed),
            start: start.map(Cow::Borrowed),
            end: end.map(Cow::Borrowed),
        }
    }
}
