use crate::prelude::*;

/// Represents a single trade history record.
///
/// Each record details an executed trade, including execution price, quantity, fees, and timestamps. Bots use this to track trade outcomes, calculate realized profits, and assess strategy effectiveness in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the trade. Bots should verify this matches the requested symbol to ensure data accuracy.
    pub symbol: String,

    /// The order type (e.g., "Limit", "Market").
    ///
    /// Indicates whether the trade was executed via a limit or market order. Bots use this to analyze execution strategy effectiveness.
    pub order_type: String,

    /// The underlying price (optional).
    ///
    /// For derivatives, this may represent the price of the underlying asset. Often empty for perpetual futures. Bots can ignore this unless analyzing specific derivative products.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub underlying_price: String,

    /// The user-defined order link ID (optional).
    ///
    /// A custom identifier for the order. Bots can use this to correlate trades with specific strategies or client orders.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub order_link_id: String,

    /// The trade side ("Buy" or "Sell").
    ///
    /// Indicates whether the trade was a buy or sell. Bots use this to track position direction and calculate net exposure.
    pub side: String,

    /// The index price at execution (optional).
    ///
    /// The index price of the asset at the time of execution, used for mark-to-market calculations in perpetual futures. Often empty in some responses. Bots can use this for P&L calculations if available.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub index_price: String,

    /// The unique order ID.
    ///
    /// Identifies the order associated with the trade. Bots use this to link trades to specific orders in their tracking systems.
    pub order_id: String,

    /// The stop order type (e.g., "StopLoss", "TakeProfit").
    ///
    /// Indicates if the trade was triggered by a stop order. Bots use this to analyze conditional order executions.
    pub stop_order_type: String,

    /// The remaining quantity after execution.
    ///
    /// The unfilled portion of the order after the trade. Bots use this to track partial fills and manage order status.
    pub leaves_qty: String,

    /// The timestamp of the trade execution.
    ///
    /// Indicates when the trade occurred. Bots use this to align trade data with other time-series data for analysis.
    pub exec_time: String,

    /// The currency of the fees (optional).
    ///
    /// Specifies the currency in which fees were charged (e.g., "USDT"). Bots use this to calculate net profitability.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub fee_currency: String,

    /// Indicates if the trade was executed as a maker.
    ///
    /// `true` if the trade was a maker order (added liquidity), `false` if taker (removed liquidity). Bots use this to optimize fee structures, as maker fees are typically lower.
    pub is_maker: bool,

    /// The execution fee for the trade.
    ///
    /// The fee charged for the trade, in the settlement currency. Bots use this to calculate net P&L and assess trading costs in perpetual futures.
    #[serde(with = "string_to_float")]
    pub exec_fee: f64,

    /// The fee rate applied to the trade.
    ///
    /// The percentage fee rate (e.g., "0.00055" for 0.055%). Bots use this to verify fee calculations and optimize for lower-cost execution strategies.
    pub fee_rate: String,

    /// The unique execution ID.
    ///
    /// A unique identifier for the trade execution on Bybit’s exchange. Bots use this to track specific trades and avoid duplicates.
    pub exec_id: String,

    /// The implied volatility for the trade (optional).
    ///
    /// Relevant for options trading, typically empty for perpetual futures. Bots can ignore this unless trading options.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub trade_iv: String,

    /// The block trade ID (optional).
    ///
    /// Identifies if the trade was a block trade, executed off the public order book. Bots can use this to detect large institutional trades.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub block_trade_id: String,

    /// The mark price at execution.
    ///
    /// The mark price used for P&L calculations in perpetual futures. Bots use this to calculate unrealized P&L and assess position health.
    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    /// The execution price of the trade.
    ///
    /// The price at which the trade was executed. Bots use this to calculate realized P&L and analyze execution quality.
    #[serde(with = "string_to_float")]
    pub exec_price: f64,

    /// The mark implied volatility (optional).
    ///
    /// Relevant for options, typically empty for perpetual futures. Bots can ignore this field.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mark_iv: String,

    /// The total order quantity.
    ///
    /// The total quantity of the order associated with the trade. Bots use this to track order size and fill progress.
    #[serde(with = "string_to_float")]
    pub order_qty: f64,

    /// The order price.
    ///
    /// The price specified in the order (for limit orders). Bots use this to compare with `exec_price` to assess slippage.
    #[serde(with = "string_to_float")]
    pub order_price: f64,

    /// The execution value of the trade.
    ///
    /// The monetary value of the trade (`exec_price` * `exec_qty`). Bots use this to calculate trade size in the settlement currency.
    #[serde(with = "string_to_float")]
    pub exec_value: f64,

    /// The execution type (e.g., "Trade", "Funding").
    ///
    /// Indicates the nature of the execution (e.g., regular trade or funding fee). Bots use this to filter trades for P&L calculations.
    pub exec_type: String,

    /// The executed quantity of the trade.
    ///
    /// The amount of the base asset traded. Bots use this to track fill amounts and update position sizes.
    #[serde(with = "string_to_float")]
    pub exec_qty: f64,

    /// The closed position size (optional).
    ///
    /// The portion of the position closed by this trade, if applicable. Bots use this to track position reductions.
    #[serde(
        rename = "closedSize",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub closed_size: String,

    /// The sequence number of the trade.
    ///
    /// A unique sequence ID for ordering trades. Bots can use this to ensure proper chronological processing of trade data.
    pub seq: u64,
}

impl<'a> TradeHistoryRequest<'a> {
    /// Creates a default TradeHistory request.
    ///
    /// Returns a request with `category` set to `Linear` and all other fields unset. Suitable for broad queries but should be customized for specific analysis needs.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> TradeHistoryRequest<'a> {
        TradeHistoryRequest::new(
            Category::Linear,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }
    /// Constructs a new TradeHistory request with specified parameters.
    ///
    /// Allows full customization. Bots should use this to specify the exact symbol, time range, and filters to align with their analysis requirements.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        base_coin: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        exec_type: Option<&'a str>,
        limit: Option<u64>,
    ) -> TradeHistoryRequest<'a> {
        TradeHistoryRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            start_time,
            end_time,
            exec_type: exec_type.map(Cow::Borrowed),
            limit,
        }
    }
}
