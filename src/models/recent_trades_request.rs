use crate::prelude::*;

/// Parameters for requesting recent trading records.
///
/// This struct defines the parameters for querying recent trades via the `/v5/market/recent-trade` endpoint. Recent trade data provides executed trade details, which are critical for analyzing market activity, liquidity, and short-term price movements in perpetual futures.
#[derive(Clone, Default)]
pub struct RecentTradesRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. For perpetual futures, use `Linear` (USDT-margined) or `Inverse` (coin-margined). Bots must set this correctly to fetch trades for the intended contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract. Bots must specify a valid symbol to retrieve trade data for the correct market.
    pub symbol: Option<Cow<'a, str>>,

    /// The base coin of the instrument (e.g., "BTC").
    ///
    /// Optionally filters trades by the base asset (e.g., all BTC-based perpetuals like `BTCUSDT` or `BTCUSD`). Useful for bots analyzing trades across multiple pairs of the same asset. If unset, trades are filtered only by `symbol`.
    pub base_coin: Option<Cow<'a, str>>,

    /// The maximum number of trade records to return (1-1000, default: 500).
    ///
    /// Controls the number of trade records in the response. Bots should set a reasonable limit to balance data completeness with performance, as large datasets can increase latency and memory usage. For high-frequency trading, a smaller limit may suffice for real-time analysis.
    pub limit: Option<u64>,
}

impl<'a> RecentTradesRequest<'a> {
    /// Creates a default RecentTrades request.
    ///
    /// Returns a request with `category` set to `Linear` and `symbol` set to `"BTCUSDT"`. Suitable for quick testing or prototyping but should be customized for production to match the specific perpetual futures contract and analysis needs.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> RecentTradesRequest<'a> {
        RecentTradesRequest::new(Category::Linear, Some("BTCUSDT"), None, None)
    }
    /// Constructs a new RecentTrades request with specified parameters.
    ///
    /// Allows full customization of the trade request. Bots should use this to specify the exact symbol, category, and other parameters to align with their trading strategy for perpetual futures.
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
    ) -> RecentTradesRequest<'a> {
        RecentTradesRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            limit,
        }
    }
}
