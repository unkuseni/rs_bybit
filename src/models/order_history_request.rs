use crate::prelude::*;

/// Represents a request to retrieve order history on Bybit.
///
/// This struct defines parameters to query past orders, filtered by time, symbol,
/// or status. In perpetual futures, order history is crucial for auditing performance
/// and refining strategies. Bots should use this to analyze executed orders and
/// adjust trading logic accordingly.
#[derive(Clone, Default)]
pub struct OrderHistoryRequest<'a> {
    /// The category of the trading product (e.g., Spot, Linear).
    ///
    /// Specifies the market to query. Bots must set this to match the target market,

    /// typically `Linear` for perpetual futures.
    pub category: Category,

    /// The trading pair symbol, e.g., "BTCUSDT".
    ///
    /// Filters orders by symbol. Bots can set this to focus on specific assets or
    /// leave it as `None` for all orders in the category.
    pub symbol: Option<Cow<'a, str>>,

    /// The base coin of the trading pair (e.g., "BTC").
    ///
    /// Filters orders by the base asset. Useful for analyzing specific assets in
    /// perpetual futures trading.
    pub base_coin: Option<Cow<'a, str>>,

    /// The settlement coin (e.g., "USDT").
    ///
    /// Filters orders by the quote asset. Useful for USDT-margined perpetual futures
    /// markets.
    pub settle_coin: Option<Cow<'a, str>>,

    /// The unique identifier of a specific order.
    ///
    /// Allows querying a single order by its Bybit-provided ID. Bots should use this
    /// for precise historical analysis.
    pub order_id: Option<Cow<'a, str>>,

    /// The user-defined identifier of a specific order.
    ///
    /// Allows querying a single order by its custom ID. Useful for tracking specific
    /// orders in perpetual futures strategies.
    pub order_link_id: Option<Cow<'a, str>>,

    /// A filter to specify the order type (e.g., "tpslOrder").
    ///
    /// Targets specific order types, such as TP/SL orders. Bots should use this to
    /// analyze relevant orders in perpetual futures.
    pub order_filter: Option<Cow<'a, str>>,

    /// The status of the orders to query (e.g., "Filled", "Cancelled").
    ///
    /// Filters orders by their execution status. Bots can use this to focus on
    /// completed or canceled orders for performance analysis in perpetual futures.
    pub order_status: Option<Cow<'a, str>>,

    /// The start time for the query (in milliseconds).
    ///
    /// Filters orders created after this timestamp. Bots should use this to narrow
    /// down historical data for specific periods in perpetual futures trading.
    pub start_time: Option<u64>,

    /// The end time for the query (in milliseconds).
    ///
    /// Filters orders created before this timestamp. Bots should pair this with
    /// `start_time` for precise time-based queries.
    pub end_time: Option<u64>,

    /// The maximum number of orders to return.
    ///
    /// Limits the response size, typically up to 50. Bots should set this to manage
    /// API response sizes and avoid rate limits in perpetual futures trading.
    pub limit: Option<u64>,
}

impl<'a> OrderHistoryRequest<'a> {
    /// Creates a default `OrderHistoryRequest` with predefined values.
    ///
    /// Initializes a query with common defaults. Bots should modify fields to target
    /// specific historical orders in perpetual futures.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            order_filter: None,
            order_status: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }
    /// Creates a custom `OrderHistoryRequest` with specified parameters.
    ///
    /// Allows bots to tailor the query fully. Ensure parameters are valid to avoid
    /// API errors in perpetual futures trading.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        order_filter: Option<&'a str>,
        order_status: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            settle_coin: settle_coin.map(Cow::Borrowed),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            order_filter: order_filter.map(Cow::Borrowed),
            order_status: order_status.map(Cow::Borrowed),
            start_time,
            end_time,
            limit,
        }
    }
}
