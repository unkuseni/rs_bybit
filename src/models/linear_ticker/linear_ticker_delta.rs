use crate::prelude::*;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LinearTickerDataDelta {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// The ONLY required field for the ticker data. Bots use this to identify the market they are trading in.
    ///
    /// Identifies the perpetual futures contract for the ticker data. Bots use this to verify the correct market.
    pub symbol: String,

    /// The tick direction of the last price change.
    ///
    /// Indicates whether the last price change was an uptick or downtick (e.g., "PlusTick"). Bots use this to analyze short-term price momentum.
    pub tick_direction: Option<TickDirection>,

    /// The 24-hour price change percentage.
    ///
    /// The percentage change in price over the last 24 hours. Bots use this to assess market trends and volatility.
    #[serde(default, with = "string_to_float_optional")]
    pub price_24h_pcnt: Option<f64>,

    /// The last traded price.
    ///
    /// The most recent price at which the contract was traded. Bots use this for real-time price tracking and technical analysis.
    #[serde(default, with = "string_to_float_optional")]
    pub last_price: Option<f64>,

    /// The price 24 hours ago.
    ///
    /// The price of the contract 24 hours prior. Bots use this to calculate price changes and validate `price_24h_pcnt`.
    #[serde(default, with = "string_to_float_optional")]
    pub prev_price_24h: Option<f64>,

    /// The highest price in the last 24 hours.
    ///
    /// The peak price reached in the last 24 hours. Bots use this to identify resistance levels and assess volatility.
    #[serde(default, with = "string_to_float_optional")]
    pub high_price_24h: Option<f64>,

    /// The lowest price in the last 24 hours.
    ///
    /// The lowest price reached in the last 24 hours. Bots use this to identify support levels and assess volatility.
    #[serde(default, with = "string_to_float_optional")]
    pub low_price_24h: Option<f64>,

    /// The price 1 hour ago.
    ///
    /// The price of the contract 1 hour prior. Bots use this to calculate short-term price changes and momentum.
    #[serde(default, with = "string_to_float_optional")]
    pub prev_price_1h: Option<f64>,

    /// The open interest value in settlement currency.
    ///
    /// The monetary value of open interest (`open_interest` * `mark_price`). Bots use this to assess market exposure and leverage levels.
    #[serde(default, with = "string_to_float_optional")]
    pub open_interest_value: Option<f64>,

    /// The 24-hour trading turnover.
    ///
    /// The total trading value in the last 24 hours, in settlement currency. Bots use this to assess market activity and liquidity.
    #[serde(default, with = "string_to_float_optional")]
    pub turnover_24h: Option<f64>,

    /// The 24-hour trading volume.
    ///
    /// The total quantity of contracts traded in the last 24 hours. Bots use this to analyze market activity and trading intensity.
    #[serde(default, with = "string_to_float_optional")]
    pub volume_24h: Option<f64>,

    /// The best bid price.
    ///
    /// The highest price at which someone is willing to buy. Bots use this to assess buy-side liquidity and calculate spreads.
    #[serde(default, rename = "bid1Price", with = "string_to_float_optional")]
    pub bid_price: Option<f64>,

    /// The best bid size.
    ///
    /// The quantity available at the best bid price. Bots use this to evaluate buy-side liquidity and potential slippage.
    #[serde(default, rename = "bid1Size", with = "string_to_float_optional")]
    pub bid_size: Option<f64>,

    /// The best ask price.
    ///
    /// The lowest price at which someone is willing to sell. Bots use this to assess sell-side liquidity and calculate spreads.
    #[serde(default, rename = "ask1Price", with = "string_to_float_optional")]
    pub ask_price: Option<f64>,

    /// The best ask size.
    ///
    /// The quantity available at the best ask price. Bots use this to evaluate sell-side liquidity and potential slippage.
    #[serde(default, rename = "ask1Size", with = "string_to_float_optional")]
    pub ask_size: Option<f64>,

    /// The pre-open price, set by Bybit for a trading pair before it enters
    /// regular trading, often during phases like subscription, announcement,
    /// or pre-trading for new pairs. This price guides early trading or
    /// subscription activities.
    #[serde(default, with = "string_to_float_optional")]
    pub pre_open_price: Option<f64>,

    /// The pre-open quantity, represents the indicative or reference price for
    /// a trading pair during a pre-listing or pre-trading phase. This price is
    /// typically set by the exchange to guide trading activity when the pair is
    /// not yet fully open for unrestricted trading (e.g., during a subscription
    /// or announcement phase for new pairs).
    #[serde(default, with = "string_to_float_optional")]
    pub pre_qty: Option<f64>,

    /// Indicates the current stage of a trading pair in Bybit’s pre-listing
    /// process. Pre-listing phases are used for new or upcoming trading pairs
    /// that are not yet fully available for trading but may be in a preparatory
    ///  or promotional stage.
    pub cur_pre_listing_phase: Option<String>,

    /// The current funding rate.
    ///
    /// The funding rate applied to positions, as a decimal (e.g., 0.0001 for 0.01%). Bots use this to calculate funding costs or profits for long/short positions.
    #[serde(default, with = "string_to_float_optional")]
    pub funding_rate: Option<f64>,

    /// The timestamp of the next funding event in milliseconds.
    ///
    /// Indicates when the next funding rate payment will occur. Bots use this to schedule funding fee calculations and position adjustments.
    #[serde(default, with = "string_to_u64_optional")]
    pub next_funding_time: Option<u64>,

    /// The current index price.
    ///
    /// The index price, based on external spot markets, used for reference in perpetual futures. Bots use this to compare with mark price for funding rate calculations.
    #[serde(default, with = "string_to_float_optional")]
    pub index_price: Option<f64>,

    /// The current mark price.
    ///
    /// The mark price used for P&L calculations in perpetual futures. Bots use this to calculate unrealized P&L and assess position health.
    #[serde(default, with = "string_to_float_optional")]
    pub mark_price: Option<f64>,

    /// The open interest in contracts.
    ///
    /// The total number of open contracts in the market. Bots use this to gauge market participation and liquidity.
    #[serde(default, with = "string_to_float_optional")]
    pub open_interest: Option<f64>,
}

impl std::fmt::Debug for LinearTickerDataDelta {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = fmt.debug_struct("LinearTickerDataDelta");

        debug_struct.field("symbol", &self.symbol);

        if let Some(index_price) = &self.index_price {
            debug_struct.field("index_price", index_price);
        }
        if let Some(tick_direction) = &self.tick_direction {
            debug_struct.field("tick_direction", tick_direction);
        }
        if let Some(price_24h_pcnt) = &self.price_24h_pcnt {
            debug_struct.field("price_24h_pcnt", price_24h_pcnt);
        }
        if let Some(last_price) = &self.last_price {
            debug_struct.field("last_price", last_price);
        }
        if let Some(prev_price_24h) = &self.prev_price_24h {
            debug_struct.field("prev_price_24h", prev_price_24h);
        }
        if let Some(high_price_24h) = &self.high_price_24h {
            debug_struct.field("high_price_24h", high_price_24h);
        }
        if let Some(low_price_24h) = &self.low_price_24h {
            debug_struct.field("low_price_24h", low_price_24h);
        }
        if let Some(prev_price_1h) = &self.prev_price_1h {
            debug_struct.field("prev_price_1h", prev_price_1h);
        }
        if let Some(mark_price) = &self.mark_price {
            debug_struct.field("mark_price", mark_price);
        }
        if let Some(open_interest) = &self.open_interest {
            debug_struct.field("open_interest", open_interest);
        }
        if let Some(open_interest_value) = &self.open_interest_value {
            debug_struct.field("open_interest_value", open_interest_value);
        }
        if let Some(turnover_24h) = &self.turnover_24h {
            debug_struct.field("turnover_24h", turnover_24h);
        }
        if let Some(volume_24h) = &self.volume_24h {
            debug_struct.field("volume_24h", volume_24h);
        }
        if let Some(next_funding_time) = &self.next_funding_time {
            debug_struct.field("next_funding_time", next_funding_time);
        }
        if let Some(funding_rate) = &self.funding_rate {
            debug_struct.field("funding_rate", funding_rate);
        }
        if let Some(bid_price) = &self.bid_price {
            debug_struct.field("bid_price", bid_price);
        }
        if let Some(bid_size) = &self.bid_size {
            debug_struct.field("bid_size", bid_size);
        }
        if let Some(ask_price) = &self.ask_price {
            debug_struct.field("ask_price", ask_price);
        }
        if let Some(ask_size) = &self.ask_size {
            debug_struct.field("ask_size", ask_size);
        }
        if let Some(pre_qty) = &self.pre_qty {
            debug_struct.field("pre_qty", pre_qty);
        }
        if let Some(cur_pre_listing_phase) = &self.cur_pre_listing_phase {
            debug_struct.field("cur_pre_listing_phase", cur_pre_listing_phase);
        }
        if let Some(pre_open_price) = &self.pre_open_price {
            debug_struct.field("pre_open_price", pre_open_price);
        }

        debug_struct.finish()
    }
}
