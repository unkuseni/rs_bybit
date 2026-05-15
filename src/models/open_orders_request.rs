use crate::prelude::*;

/// Represents a request to retrieve open orders on Bybit.
///
/// This struct defines parameters to query active orders, filtered by symbol, category,
/// or other criteria. In perpetual futures, monitoring open orders is critical for
/// position management and strategy adjustments. Bots should use this to track pending
/// orders and avoid over-leveraging or unintended executions.
#[derive(Clone, Default)]
pub struct OpenOrdersRequest<'a> {
    /// The category of the trading product (e.g., Spot, Linear).
    ///
    /// Specifies the market to query. Bots must set this to match the target market,

    /// typically `Linear` for perpetual futures.
    pub category: Category,

    /// The trading pair symbol, e.g., "BTCUSDT".
    ///
    /// Filters orders by symbol. Bots can set this to focus on specific assets or leave
    /// it broad to retrieve all orders in the category.
    pub symbol: Cow<'a, str>,

    /// The base coin of the trading pair (e.g., "BTC").
    ///
    /// Filters orders by the base asset. Useful for bots managing multiple pairs in
    /// perpetual futures, allowing targeted queries.
    pub base_coin: Option<Cow<'a, str>>,

    /// The settlement coin (e.g., "USDT").
    ///
    /// Filters orders by the quote asset. Bots can use this to narrow down queries in
    /// USDT-margined perpetual futures markets.
    pub settle_coin: Option<Cow<'a, str>>,

    /// The unique identifier of a specific order.
    ///
    /// Allows querying a single order by its Bybit-provided ID. Bots should use this
    /// for precise order tracking in perpetual futures.
    pub order_id: Option<Cow<'a, str>>,

    /// The user-defined identifier of a specific order.
    ///
    /// Allows querying a single order by its custom ID. Useful for bots tracking orders
    /// across multiple strategies in perpetual futures.
    pub order_link_id: Option<Cow<'a, str>>,

    /// Filters orders by open status (0, 1, or 2).
    ///
    /// Specifies whether to retrieve all orders (0), open orders (1), or partially
    /// filled orders (2). Bots should set this based on monitoring needs in perpetual
    /// futures trading.
    pub open_only: Option<usize>,

    /// A filter to specify the order type (e.g., "tpslOrder").
    ///
    /// Targets specific order types, such as TP/SL orders. Bots should use this to
    /// focus on relevant orders in perpetual futures risk management.
    pub order_filter: Option<Cow<'a, str>>,

    /// The maximum number of orders to return.
    ///
    /// Limits the response size, typically up to 50. Bots should set this to manage
    /// API response sizes and avoid rate limits in perpetual futures trading.
    pub limit: Option<usize>,
}

impl<'a> OpenOrdersRequest<'a> {
    /// Creates a default `OpenOrdersRequest` with predefined values.
    ///
    /// Initializes a query with common defaults. Bots should modify fields as needed
    /// to target specific orders in perpetual futures.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed("BTCUSDT"),
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            open_only: None,
            order_filter: None,
            limit: None,
        }
    }
    /// Creates a custom `OpenOrdersRequest` with specified parameters.
    ///
    /// Allows bots to tailor the query fully. Ensure parameters are valid to avoid
    /// API errors, and set `open_only` within the allowed range (0–2) for perpetual
    /// futures trading.
    #[allow(clippy::too_many_arguments)]
    pub fn custom(
        category: Category,
        symbol: &'a str,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        open_only: usize,
        order_filter: Option<&'a str>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            base_coin: base_coin.map(Cow::Borrowed),
            settle_coin: settle_coin.map(Cow::Borrowed),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            open_only: match open_only {
                0..=2 => Some(open_only),
                _ => None,
            },
            order_filter: order_filter.map(Cow::Borrowed),
            limit,
        }
    }
}
