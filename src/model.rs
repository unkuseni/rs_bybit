#![allow(unused_imports)]
use crate::errors::BybitError;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{from_value, Value};
use std::{borrow::Cow, collections::BTreeMap};
use thiserror::Error;

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Empty {}

/// ----------------------------------------
///  RESPONSE STRUCTS FOR MARKET REQUESTS
/// ----------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: ServerTime,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    #[serde(with = "string_to_u64")]
    pub time_second: u64,
    #[serde(with = "string_to_u64")]
    pub time_nano: u64,
}

#[derive(Clone, Default)]
pub struct KlineRequest<'a> {
    pub category: Option<Category>,
    pub symbol: Cow<'a, str>,
    pub interval: Cow<'a, str>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> KlineRequest<'a> {
    pub fn default() -> KlineRequest<'a> {
        KlineRequest::new(None, "BTCUSDT", "", None, None, None)
    }
    pub fn new(
        category: Option<Category>,
        symbol: &'a str,
        interval: &'a str,
        start: Option<&'a str>,
        end: Option<&'a str>,
        limit: Option<u64>,
    ) -> KlineRequest<'a> {
        KlineRequest {
            category: category,
            symbol: Cow::Borrowed(symbol),
            interval: Cow::Borrowed(interval),
            start: start.map(|s| Cow::Borrowed(s)),
            end: end.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: KlineSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<Kline>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub quote_asset_volume: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: MarkPriceKlineSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<MarkPriceKline>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKline {
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: IndexPriceKlineSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<IndexPriceKline>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKline {
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: PremiumIndexPriceKlineSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKlineSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<PremiumIndexPriceKline>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexPriceKline {
    #[serde(with = "string_to_u64")]
    pub start_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
}

#[derive(Clone, Default)]
pub struct InstrumentRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub status: Option<bool>,
    pub base_coin: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}
impl<'a> InstrumentRequest<'a> {
    pub fn default() -> InstrumentRequest<'a> {
        InstrumentRequest::new(Category::Linear, Some("BTCUSDT"), None, None, None)
    }
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        status: Option<bool>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest {
            category: category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            status: status,
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrumentsInfoResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: FuturesInstrumentsInfo,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrumentsInfo {
    pub category: String,
    pub list: Vec<FuturesInstrument>,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesInstrument {
    pub symbol: String,
    #[serde(rename = "contractType")]
    pub contract_type: String,
    pub status: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    #[serde(rename = "launchTime", with = "string_to_u64")]
    pub launch_time: u64,
    #[serde(rename = "deliveryTime", skip_serializing_if = "String::is_empty")]
    pub delivery_time: String,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String,
    #[serde(rename = "priceScale")]
    pub price_scale: String,
    #[serde(rename = "leverageFilter")]
    pub leverage_filter: LeverageFilter,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
    #[serde(rename = "unifiedMarginTrade")]
    pub unified_margin_trade: bool,
    #[serde(rename = "fundingInterval")]
    pub funding_interval: u64,
    #[serde(rename = "settleCoin")]
    pub settle_coin: String,
    #[serde(rename = "copyTrading")]
    pub copy_trading: String,
    #[serde(rename = "upperFundingRate")]
    pub upper_funding_rate: String,
    #[serde(rename = "lowerFundingRate")]
    pub lower_funding_rate: String,
    // #[serde(rename = "isPreListing" )]
    // pub is_pre_listing: bool,
    // #[serde(rename = "preListingInfo")]
    // pub pre_listing_info: PreListingInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfoResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: SpotInstrumentsInfo,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfo {
    pub category: String,
    pub list: Vec<SpotInstrument>,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrument {
    pub symbol: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    pub innovation: String,
    pub status: String,
    #[serde(rename = "marginTrading")]
    pub margin_trading: String,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "riskParameters")]
    pub risk_parameters: RiskParameters,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionsInstrument {
    pub symbol: String,
    pub status: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    #[serde(rename = "settleCoin")]
    pub settle_coin: String,
    #[serde(rename = "optionType")]
    pub option_type: String,
    #[serde(rename = "launchTime", with = "string_to_u64")]
    pub launch_time: u64,
    #[serde(rename = "deliveryTime", with = "string_to_u64")]
    pub delivery_time: u64,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreListingInfo {
    pub cur_auction_phase: String,
    pub phases: Vec<PreListingPhase>,
    pub auction_fee_info: AuctionFeeInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreListingPhase {
    pub phase: String,
    #[serde(rename = "startTime", with = "string_to_u64")]
    pub start_time: u64,
    #[serde(rename = "endTime", with = "string_to_u64")]
    pub end_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuctionFeeInfo {
    pub auction_fee_rate: String,
    pub taker_fee_rate: String,
    pub maker_fee_rate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    #[serde(rename = "limitParameter")]
    pub limit_parameter: String,
    #[serde(rename = "marketParameter")]
    pub market_parameter: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    #[serde(rename = "minLeverage")]
    pub min_leverage: String,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: String,
    #[serde(rename = "leverageStep")]
    pub leverage_step: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,
    #[serde(rename = "tickSize", with = "string_to_float")]
    pub tick_size: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    #[serde(
        rename = "basePrecision",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub base_precision: Option<String>,
    #[serde(rename = "quotePrecision", skip_serializing_if = "Option::is_none")]
    pub quote_precision: Option<String>,
    #[serde(rename = "maxMktOrderQty", skip_serializing_if = "Option::is_none")]
    pub max_mkt_order_qty: Option<String>,
    #[serde(rename = "minOrderQty", with = "string_to_float")]
    pub min_order_qty: f64,
    #[serde(rename = "maxOrderQty", with = "string_to_float")]
    pub max_order_qty: f64,
    #[serde(rename = "minOrderAmt", skip_serializing_if = "Option::is_none")]
    pub min_order_amt: Option<String>,
    #[serde(rename = "maxOrderAmt", skip_serializing_if = "Option::is_none")]
    pub max_order_amt: Option<String>,
    #[serde(rename = "qtyStep", skip_serializing_if = "Option::is_none")]
    pub qty_step: Option<String>,
    #[serde(
        rename = "postOnlyMaxOrderQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_only_max_order_qty: Option<String>,
    #[serde(rename = "minNotionalValue", skip_serializing_if = "Option::is_none")]
    pub min_notional_value: Option<String>,
}

#[derive(Clone, Default)]
pub struct OrderbookRequest<'a> {
    pub symbol: Cow<'a, str>,
    pub category: Category,
    pub limit: Option<u64>,
}

impl<'a> OrderbookRequest<'a> {
    pub fn default() -> OrderbookRequest<'a> {
        OrderbookRequest::new("BTCUSDT", Category::Linear, None)
    }

    pub fn new(symbol: &'a str, category: Category, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest {
            symbol: Cow::Borrowed(symbol),
            category,
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: OrderBook,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde(rename = "u")]
    pub update_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(with = "string_to_float")]
    pub qty: f64,
}

impl Bid {
    pub fn new(price: f64, qty: f64) -> Bid {
        Bid { price, qty }
    }
}
impl Ask {
    pub fn new(price: f64, qty: f64) -> Ask {
        Ask { price, qty }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTickersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: FuturesTickers,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTickersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: SpotTickers,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTickers {
    pub category: String,
    pub list: Vec<FuturesTicker>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTickers {
    pub category: String,
    pub list: Vec<SpotTicker>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTicker {
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    #[serde(with = "string_to_float")]
    pub index_price: f64,
    #[serde(with = "string_to_float")]
    pub mark_price: f64,
    #[serde(rename = "prevPrice24h", with = "string_to_float")]
    pub prev_price_24h: f64,
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,
    #[serde(rename = "prevPrice1h", with = "string_to_float")]
    pub prev_price_1h: f64,
    #[serde(with = "string_to_float")]
    pub open_interest: f64,
    #[serde(with = "string_to_float")]
    pub open_interest_value: f64,
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
    pub funding_rate: String,
    #[serde(rename = "nextFundingTime", with = "string_to_u64")]
    pub next_funding_time: u64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub predicted_delivery_price: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub basis_rate: String,
    pub delivery_fee_rate: String,
    #[serde(rename = "deliveryTime", with = "string_to_u64")]
    pub delivery_time: u64,
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub basis: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    pub symbol: String,
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,
    #[serde(with = "string_to_float")]
    pub last_price: f64,
    #[serde(rename = "prevPrice24h", with = "string_to_float")]
    pub prev_price_24h: f64,
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
    #[serde(rename = "usdIndexPrice")]
    pub usd_index_price: String,
}

#[derive(Clone, Default)]
pub struct FundingHistoryRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
}
impl<'a> FundingHistoryRequest<'a> {
    pub fn default() -> FundingHistoryRequest<'a> {
        FundingHistoryRequest::new(Category::Linear, "BTCUSDT", None, None, None)
    }
    pub fn new(
        category: Category,
        symbol: &'a str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> FundingHistoryRequest<'a> {
        FundingHistoryRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            start_time,
            end_time,
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: FundingRateSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateSummary {
    pub category: String,
    pub list: Vec<FundingRate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    #[serde(rename = "fundingRate", with = "string_to_float")]
    pub funding_rate: f64,
    #[serde(rename = "fundingRateTimestamp", with = "string_to_u64")]
    pub funding_rate_timestamp: u64,
}

#[derive(Clone, Default)]
pub struct RecentTradesRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}
impl<'a> RecentTradesRequest<'a> {
    pub fn default() -> RecentTradesRequest<'a> {
        RecentTradesRequest::new(Category::Linear, Some("BTCUSDT"), None, None)
    }
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
    ) -> RecentTradesRequest<'a> {
        RecentTradesRequest {
            category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: RecentTrades,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrades {
    pub category: String,
    pub list: Vec<RecentTrade>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
    #[serde(rename = "execId")]
    pub exec_id: String,
    pub symbol: String,
    #[serde(with = "string_to_float")]
    pub price: f64,
    #[serde(rename = "size", with = "string_to_float")]
    pub qty: f64,
    pub side: String,
    #[serde(rename = "time")]
    pub timestamp: String,
    #[serde(rename = "isBlockTrade")]
    pub is_block_trade: bool,
}

#[derive(Clone, Default)]
pub struct OpenInterestRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub interval: Cow<'a, str>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> OpenInterestRequest<'a> {
    pub fn default() -> OpenInterestRequest<'a> {
        OpenInterestRequest::new(Category::Linear, "BTCUSDT", "4h", None, None, None)
    }
    pub fn new(
        category: Category,
        symbol: &'a str,
        interval: &'a str,
        start: Option<&'a str>,
        end: Option<&'a str>,
        limit: Option<u64>,
    ) -> OpenInterestRequest<'a> {
        OpenInterestRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            interval: Cow::Borrowed(interval),
            start: start.map(|s| Cow::Borrowed(s)),
            end: end.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpeninterestResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: OpenInterestSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestSummary {
    pub symbol: String,
    pub category: String,
    pub list: Vec<OpenInterest>,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(rename = "openInterest", with = "string_to_float")]
    pub open_interest: f64,
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,
}

#[derive(Clone, Default)]
pub struct HistoricalVolatilityRequest<'a> {
    pub base_coin: Option<Cow<'a, str>>,
    pub period: Option<Cow<'a, str>>,
    pub start: Option<Cow<'a, str>>,
    pub end: Option<Cow<'a, str>>,
}

impl<'a> HistoricalVolatilityRequest<'a> {
    pub fn default() -> HistoricalVolatilityRequest<'a> {
        HistoricalVolatilityRequest::new(Some("BTC"), None, None, None)
    }
    pub fn new(
        base_coin: Option<&'a str>,
        period: Option<&'a str>,
        start: Option<&'a str>,
        end: Option<&'a str>,
    ) -> HistoricalVolatilityRequest<'a> {
        HistoricalVolatilityRequest {
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            period: period.map(|s| Cow::Borrowed(s)),
            start: start.map(|s| Cow::Borrowed(s)),
            end: end.map(|s| Cow::Borrowed(s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatilityResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub category: String,
    #[serde(rename = "result")]
    pub result: Vec<HistoricalVolatility>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatility {
    pub period: u64,
    #[serde(with = "string_to_float")]
    pub value: f64,
    #[serde(rename = "time", with = "string_to_u64")]
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: InsuranceSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceSummary {
    #[serde(rename = "updatedTime", with = "string_to_u64")]
    pub updated_time: u64,
    pub list: Vec<Insurance>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Insurance {
    pub coin: String,
    #[serde(with = "string_to_float")]
    pub balance: f64,
    pub value: String,
}

#[derive(Clone, Default)]
pub struct RiskLimitRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
}

impl<'a> RiskLimitRequest<'a> {
    pub fn default() -> RiskLimitRequest<'a> {
        RiskLimitRequest::new(Category::Linear, None)
    }
    pub fn new(category: Category, symbol: Option<&'a str>) -> RiskLimitRequest<'a> {
        RiskLimitRequest {
            category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RiskLimitResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: RiskLimitSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RiskLimitSummary {
    pub category: String,
    pub list: Vec<RiskLimit>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RiskLimit {
    pub id: u64,
    pub symbol: String,
    #[serde(rename = "riskLimitValue", with = "string_to_float")]
    pub risk_limit_value: f64,
    #[serde(rename = "maintenanceMargin", with = "string_to_float")]
    pub maintainence_margin: f64,
    #[serde(rename = "initialMargin", with = "string_to_float")]
    pub initial_margin: f64,
    #[serde(rename = "isLowestRisk")]
    pub is_lowest_risk: u8,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: DeliveryPriceSummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceSummary {
    pub category: String,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    pub list: Vec<DeliveryPrice>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    pub symbol: String,
    pub delivery_price: String,
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: LongShortRatioSummary,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioSummary {
    pub list: Vec<LongShortRatio>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "buyRatio", with = "string_to_float")]
    pub buy_ratio: f64,
    #[serde(rename = "sellRatio", with = "string_to_float")]
    pub sell_ratio: f64,
    #[serde(rename = "timestamp", with = "string_to_u64")]
    pub timestamp: u64,
}

/// --------------------------------------------------
///  REQUEST & RESPONSE STRUCTS FOR TRADE
/// --------------------------------------------------
#[derive(Clone, Copy, Default, Serialize)]
pub enum Category {
    Spot,
    #[default]
    Linear,
    Inverse,
    Option,
}
impl Category {
    pub fn as_str(&self) -> &str {
        match self {
            Category::Spot => "spot",
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            Category::Option => "option",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}

impl Side {
    pub fn as_str(&self) -> &str {
        match self {
            Side::Buy => "Buy",
            Side::Sell => "Sell",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum OrderType {
    Limit,
    #[default]
    Market,
}

impl OrderType {
    pub fn as_str(&self) -> &str {
        match self {
            OrderType::Limit => "Limit",
            OrderType::Market => "Market",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum TimeInForce {
    #[default]
    GTC,
    IOC,
    FOK,
    PostOnly,
}

impl TimeInForce {
    pub fn as_str(&self) -> &str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
            TimeInForce::PostOnly => "PostOnly",
        }
    }
}
#[derive(Clone, Default, Serialize)]
pub struct OrderRequest<'a> {
    pub category: Category,                 // String
    pub symbol: Cow<'a, str>,               // String
    pub is_leverage: Option<bool>,          // Integer
    pub side: Side,                         // String
    pub order_type: OrderType,              // String
    pub qty: f64,                           // String
    pub market_unit: Option<f64>,           // String
    pub price: Option<f64>,                 // String
    pub trigger_direction: Option<bool>,    // String
    pub order_filter: Option<Cow<'a, str>>, // String
    pub trigger_price: Option<f64>,
    pub trigger_by: Option<Cow<'a, str>>,    // String
    pub order_iv: Option<f64>,               // String
    pub time_in_force: Option<Cow<'a, str>>, // String
    pub position_idx: Option<u8>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<Cow<'a, str>>,
    pub sl_trigger_by: Option<Cow<'a, str>>,
    pub reduce_only: Option<bool>,
    pub close_on_trigger: Option<bool>,
    pub smp_type: Option<Cow<'a, str>>,
    pub mmp: Option<bool>,
    pub tpsl_mode: Option<Cow<'a, str>>,
    pub tp_limit_price: Option<f64>,
    pub sl_limit_price: Option<f64>,
    pub tp_order_type: Option<Cow<'a, str>>,
    pub sl_order_type: Option<Cow<'a, str>>,
}

impl<'a> OrderRequest<'a> {
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed("BTCUSDT"),
            is_leverage: None,
            side: Side::default(),
            order_type: OrderType::Market,
            qty: 0.00,
            market_unit: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }
    pub fn custom(
        category: Category,
        symbol: &'a str,
        leverage: Option<bool>,
        side: Side,
        order_type: OrderType,
        qty: f64,
        market_unit: Option<f64>,
        price: Option<f64>,
        trigger_direction: Option<bool>,
        order_filter: Option<&'a str>,
        trigger_price: Option<f64>,
        trigger_by: Option<&'a str>,
        order_iv: Option<f64>,
        time_in_force: Option<&'a str>,
        position_idx: Option<u8>,
        order_link_id: Option<&'a str>,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        reduce_only: Option<bool>,
        close_on_trigger: Option<bool>,
        smp_type: Option<&'a str>,
        mmp: Option<bool>,
        tpsl_mode: Option<&'a str>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
        tp_order_type: Option<&'a str>,
        sl_order_type: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            is_leverage: leverage,
            side,
            order_type,
            qty,
            market_unit,
            price,
            trigger_direction,
            order_filter: order_filter.map(Cow::Borrowed),
            trigger_price,
            trigger_by: trigger_by.map(Cow::Borrowed),
            order_iv,
            time_in_force: time_in_force.map(Cow::Borrowed),
            position_idx,
            order_link_id: order_link_id.map(Cow::Borrowed),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            reduce_only,
            close_on_trigger,
            smp_type: smp_type.map(Cow::Borrowed),
            mmp,
            tpsl_mode: tpsl_mode.map(Cow::Borrowed),
            tp_limit_price,
            sl_limit_price,
            tp_order_type: tp_order_type.map(Cow::Borrowed),
            sl_order_type: sl_order_type.map(Cow::Borrowed),
        }
    }
    pub fn spot_limit_with_market_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tp_order_type: Some(Cow::Borrowed("Market")),
            sl_order_type: Some(Cow::Borrowed("Market")),
            ..Self::default()
        }
    }
    pub fn spot_limit_with_limit_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tp_limit_price: Some(tp),
            sl_limit_price: Some(sl),
            tp_order_type: Some(Cow::Borrowed("Limit")),
            sl_order_type: Some(Cow::Borrowed("Limit")),
            ..Self::default()
        }
    }
    pub fn spot_postonly(symbol: &'a str, side: Side, qty: f64, price: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            ..Self::default()
        }
    }
    pub fn spot_tpsl(
        symbol: &'a str,
        side: Side,
        price: f64,
        qty: f64,
        order_link_id: Option<&'a str>,
    ) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::GTC.as_str())),
            order_link_id: order_link_id.map(Cow::Borrowed),
            order_filter: Some(Cow::Borrowed("tpslOrder")),
            ..Self::default()
        }
    }
    pub fn spot_margin(symbol: &'a str, side: Side, qty: f64, price: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            is_leverage: Some(true),
            ..Self::default()
        }
    }

    pub fn spot_market(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            ..Self::default()
        }
    }

    pub fn futures_limit_with_market_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            reduce_only: Some(false),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tpsl_mode: Some(Cow::Borrowed("Full")),
            tp_order_type: Some(Cow::Borrowed("Market")),
            sl_order_type: Some(Cow::Borrowed("Market")),
            ..Self::default()
        }
    }

    pub fn futures_limit_with_limit_tpsl(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::PostOnly.as_str())),
            reduce_only: Some(false),
            take_profit: Some(tp),
            stop_loss: Some(sl),
            tpsl_mode: Some(Cow::Borrowed("Partial")),
            tp_order_type: Some(Cow::Borrowed("Limit")),
            sl_order_type: Some(Cow::Borrowed("Limit")),
            tp_limit_price: Some(tp),
            sl_limit_price: Some(sl),
            ..Self::default()
        }
    }

    pub fn futures_market(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            reduce_only: Some(false),
            ..Self::default()
        }
    }

    pub fn futures_close_limit(
        symbol: &'a str,
        side: Side,
        qty: f64,
        price: f64,
        order_link_id: &'a str,
    ) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Limit,
            qty,
            price: Some(price),
            time_in_force: Some(Cow::Borrowed(TimeInForce::GTC.as_str())),
            order_link_id: Some(Cow::Borrowed(order_link_id)),
            reduce_only: Some(true),
            ..Self::default()
        }
    }

    pub fn futures_market_close(symbol: &'a str, side: Side, qty: f64) -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed(symbol),
            side,
            order_type: OrderType::Market,
            qty,
            time_in_force: Some(Cow::Borrowed(TimeInForce::IOC.as_str())),
            reduce_only: Some(true),
            ..Self::default()
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: OrderStatus,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Clone, Default, Serialize)]
pub struct AmendOrderRequest<'a> {
    pub category: Category,   // String
    pub symbol: Cow<'a, str>, // String
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub order_iv: Option<f64>, // String
    pub trigger_price: Option<f64>,
    pub qty: f64,           // String
    pub price: Option<f64>, // String
    pub tpsl_mode: Option<Cow<'a, str>>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<Cow<'a, str>>,
    pub sl_trigger_by: Option<Cow<'a, str>>,
    pub trigger_by: Option<Cow<'a, str>>, // String
    pub tp_limit_price: Option<f64>,
    pub sl_limit_price: Option<f64>,
}

impl<'a> AmendOrderRequest<'a> {
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: Cow::Borrowed("BTCUSDT"),
            order_id: None,
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: 0.00,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }
    pub fn custom(
        category: Category,
        symbol: &'a str,
        order_id: Option<&'a str>,
        order_link_id: Option<&'a str>,
        order_iv: Option<f64>,
        trigger_price: Option<f64>,
        qty: f64,
        price: Option<f64>,
        tpsl_mode: Option<&'a str>,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        trigger_by: Option<&'a str>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            order_id: order_id.map(Cow::Borrowed),
            order_link_id: order_link_id.map(Cow::Borrowed),
            order_iv,
            trigger_price,
            qty,
            price,
            tpsl_mode: tpsl_mode.map(Cow::Borrowed),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(Cow::Borrowed),
            sl_trigger_by: sl_trigger_by.map(Cow::Borrowed),
            trigger_by: trigger_by.map(Cow::Borrowed),
            tp_limit_price,
            sl_limit_price,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct CancelOrderRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub order_filter: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: OrderStatus,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct OpenOrdersRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub base_coin: Option<Cow<'a, str>>,
    pub settle_coin: Option<Cow<'a, str>>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub open_only: Option<usize>,
    pub order_filter: Option<Cow<'a, str>>,
    pub limit: Option<usize>,
}

impl<'a> OpenOrdersRequest<'a> {
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
                0 | 1 | 2 => Some(open_only),
                _ => None,
            },
            order_filter: order_filter.map(Cow::Borrowed),
            limit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: OrderHistory,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: OrderStatus,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct OrderHistoryRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub settle_coin: Option<Cow<'a, str>>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub order_filter: Option<Cow<'a, str>>,
    pub order_status: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
}

impl<'a> OrderHistoryRequest<'a> {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: OrderHistory,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistory {
    pub category: String,
    pub list: Vec<Order>,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub order_link_id: String,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub block_trade_id: Option<String>,

    pub symbol: String,

    #[serde(with = "string_to_float")]
    pub price: f64,

    #[serde(with = "string_to_float")]
    pub qty: f64,

    pub side: Side,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub is_leverage: String,

    pub position_idx: i32,
    pub order_status: String,
    pub cancel_type: String,
    pub reject_reason: String,

    #[serde(with = "string_to_float_optional")]
    pub avg_price: Option<f64>,

    #[serde(with = "string_to_float")]
    pub leaves_qty: f64,

    #[serde(with = "string_to_float")]
    pub leaves_value: f64,

    #[serde(with = "string_to_float")]
    pub cum_exec_qty: f64,

    #[serde(with = "string_to_float")]
    pub cum_exec_value: f64,

    #[serde(with = "string_to_float")]
    pub cum_exec_fee: f64,

    pub time_in_force: String,
    pub order_type: OrderType,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub stop_order_type: Option<String>,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub order_iv: Option<String>,

    #[serde(with = "string_to_float_optional")]
    pub trigger_price: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub take_profit: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub stop_loss: Option<f64>,

    pub tp_trigger_by: String,
    pub sl_trigger_by: String,
    pub trigger_direction: i32,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub trigger_by: Option<String>,

    #[serde(with = "string_to_float_optional")]
    pub last_price_on_created: Option<f64>,

    pub reduce_only: bool,
    pub close_on_trigger: bool,
    pub smp_type: String,
    pub smp_group: i32,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub smp_order_id: Option<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub tpsl_mode: String,

    #[serde(with = "string_to_float_optional")]
    pub tp_limit_price: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub sl_limit_price: Option<f64>,

    #[serde(
        deserialize_with = "empty_string_as_none",
        skip_serializing_if = "is_empty_or_none"
    )]
    pub place_type: Option<String>,

    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}

// Custom function to check if Option<String> is None or Some("")
fn is_empty_or_none(opt: &Option<String>) -> bool {
    opt.as_ref().map_or(true, |s| s.is_empty())
}

// Custom deserialization function to treat empty strings as None
fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize into a String first
    let s = String::deserialize(deserializer)?;
    // Return None if the string is empty, otherwise Some(s)
    Ok(if s.is_empty() { None } else { Some(s) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_orders() {
        let json = r#"
        {
                "orderId": "fd4300ae-7847-404e-b947-b46980a4d140",
                "orderLinkId": "test-000005",
                "blockTradeId": "",
                "symbol": "ETHUSDT",
                "price": "1600.00",
                "qty": "0.10",
                "side": "Buy",
                "isLeverage": "",
                "positionIdx": 1,
                "orderStatus": "New",
                "cancelType": "UNKNOWN",
                "rejectReason": "EC_NoError",
                "avgPrice": "0",
                "leavesQty": "0.10",
                "leavesValue": "160",
                "cumExecQty": "0.00",
                "cumExecValue": "0",
                "cumExecFee": "0",
                "timeInForce": "GTC",
                "orderType": "Limit",
                "stopOrderType": "UNKNOWN",
                "orderIv": "",
                "triggerPrice": "0.00",
                "takeProfit": "2500.00",
                "stopLoss": "1500.00",
                "tpTriggerBy": "LastPrice",
                "slTriggerBy": "LastPrice",
                "triggerDirection": 0,
                "triggerBy": "UNKNOWN",
                "lastPriceOnCreated": "",
                "reduceOnly": false,
                "closeOnTrigger": false,
                "smpType": "None",
                "smpGroup": 0,
                "smpOrderId": "",
                "tpslMode": "Full",
                "tpLimitPrice": "",
                "slLimitPrice": "",
                "placeType": "",
                "createdTime": "1684738540559",
                "updatedTime": "1684738540561"
            }
        "#;
        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.order_id, "fd4300ae-7847-404e-b947-b46980a4d140");
    }
}

#[derive(Clone, Default)]
pub struct CancelallRequest<'a> {
    pub category: Category,
    pub symbol: &'a str,
    pub base_coin: Option<&'a str>,
    pub settle_coin: Option<&'a str>,
    pub order_filter: Option<&'a str>,
    pub stop_order_type: Option<&'a str>,
}

impl<'a> CancelallRequest<'a> {
    pub fn default() -> Self {
        Self {
            category: Category::Linear,
            symbol: "BTCUSDT",
            base_coin: None,
            settle_coin: None,
            order_filter: None,
            stop_order_type: None,
        }
    }
    pub fn new(
        category: Category,
        symbol: &'a str,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        order_filter: Option<&'a str>,
        stop_order_type: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol,
            base_coin,
            settle_coin,
            order_filter,
            stop_order_type,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelallResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: CancelledList,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelledList {
    pub list: Vec<OrderStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistoryResponse {
    pub ret_code: i16,
    pub ret_msg: String,
    pub result: TradeHistorySummary,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistorySummary {
    #[serde(rename = "nextPageCursor", skip_serializing_if = "String::is_empty")]
    pub next_page_cursor: String,
    pub category: String,
    pub list: Vec<TradeHistory>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub symbol: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(
        rename = "underlyingPrice",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub underlying_price: String,
    #[serde(
        rename = "orderLinkId",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub order_link_id: String,
    pub side: String,
    #[serde(
        rename = "indexPrice",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub index_price: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,
    #[serde(rename = "leavesQty")]
    pub leaves_qty: String,
    #[serde(rename = "execTime")]
    pub exec_time: String,
    #[serde(
        rename = "feeCurrency",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub fee_currency: String,
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
    #[serde(rename = "execFee")]
    pub exec_fee: String,
    #[serde(rename = "feeRate")]
    pub fee_rate: String,
    #[serde(rename = "execId")]
    pub exec_id: String,
    #[serde(rename = "tradeIv", default, skip_serializing_if = "String::is_empty")]
    pub trade_iv: String,
    #[serde(
        rename = "blockTradeId",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub block_trade_id: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "execPrice")]
    pub exec_price: String,
    #[serde(rename = "markIv", default, skip_serializing_if = "String::is_empty")]
    pub mark_iv: String,
    #[serde(rename = "orderQty")]
    pub order_qty: String,
    #[serde(rename = "orderPrice")]
    pub order_price: String,
    #[serde(rename = "execValue")]
    pub exec_value: String,
    #[serde(rename = "execType")]
    pub exec_type: String,
    #[serde(rename = "execQty")]
    pub exec_qty: String,
    #[serde(
        rename = "closedSize",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub closed_size: String,
    pub seq: u64,
}

#[derive(Clone, Default)]
pub struct TradeHistoryRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub order_id: Option<Cow<'a, str>>,
    pub order_link_id: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub exec_type: Option<Cow<'a, str>>,
    pub limit: Option<u64>,
}

impl<'a> TradeHistoryRequest<'a> {
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
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            order_id: order_id.map(|s| Cow::Borrowed(s)),
            order_link_id: order_link_id.map(|s| Cow::Borrowed(s)),
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            start_time,
            end_time,
            exec_type: exec_type.map(|s| Cow::Borrowed(s)),
            limit,
        }
    }
}

#[derive(Clone, Default)]
pub struct BatchPlaceRequest<'a> {
    pub category: Category,
    pub requests: Vec<OrderRequest<'a>>,
}
impl<'a> BatchPlaceRequest<'a> {
    pub fn new(category: Category, requests: Vec<OrderRequest<'a>>) -> BatchPlaceRequest<'a> {
        BatchPlaceRequest { category, requests }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchPlaceResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: BatchedOrderList,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: OrderConfirmationList,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchedOrderList {
    pub list: Vec<BatchedOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchedOrder {
    pub category: String,
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
    #[serde(rename = "createAt")]
    pub create_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderConfirmationList {
    pub list: Vec<OrderConfirmation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderConfirmation {
    pub code: i16,
    pub msg: String,
}

#[derive(Clone, Default)]
pub struct BatchAmendRequest<'a> {
    pub category: Category,
    pub requests: Vec<AmendOrderRequest<'a>>,
}

impl<'a> BatchAmendRequest<'a> {
    pub fn new(category: Category, requests: Vec<AmendOrderRequest<'a>>) -> BatchAmendRequest<'a> {
        BatchAmendRequest { category, requests }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: AmendedOrderList,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: OrderConfirmationList,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmendedOrderList {
    pub list: Vec<AmendedOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmendedOrder {
    pub category: String,
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
}

#[derive(Clone, Default)]
pub struct BatchCancelRequest<'a> {
    pub category: Category,
    pub requests: Vec<CancelOrderRequest<'a>>,
}

impl<'a> BatchCancelRequest<'a> {
    pub fn new(
        category: Category,
        requests: Vec<CancelOrderRequest<'a>>,
    ) -> BatchCancelRequest<'a> {
        BatchCancelRequest { category, requests }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: CanceledOrderList,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: OrderConfirmationList,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrderList {
    pub list: Vec<CanceledOrder>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub category: String,
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
}

#[derive(Clone)]
pub enum RequestType<'a> {
    Create(BatchPlaceRequest<'a>),
    Amend(BatchAmendRequest<'a>),
    Cancel(BatchCancelRequest<'a>),
}

// ----------------------------------------------------------
// POSITION STRUCTS SECTION
// ------------------------------------------------------

#[derive(Clone, Default)]
pub struct PositionRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub settle_coin: Option<Cow<'a, str>>,
    pub limit: Option<usize>,
}

impl<'a> PositionRequest<'a> {
    pub fn default() -> Self {
        Self::new(Category::Linear, None, None, None, None)
    }
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            settle_coin: settle_coin.map(Cow::Borrowed),
            limit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: InfoResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoResult {
    pub list: Vec<PositionInfo>,
    #[serde(rename = "nextPageCursor", skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    pub category: String,
}

mod string_to_float_default_zero {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    // Serialization: Convert f64 to string
    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    // Deserialization: Parse string to f64, return 0.0 for empty string
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(0.0) // Return 0.0 for empty string
        } else {
            f64::from_str(&s).map_err(serde::de::Error::custom)
        }
    }
}

mod string_to_float_optional {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    // Serialization: Convert Option<f64> to string
    pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_str(""),
        }
    }

    // Deserialization: Parse string to Option<f64>, return None for empty string
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None) // Return None for empty string
        } else {
            f64::from_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    pub position_idx: i32,

    pub risk_id: i32,

    #[serde(with = "string_to_float")]
    pub risk_limit_value: f64,

    pub symbol: String,

    pub side: String,

    #[serde(with = "string_to_float")]
    pub size: f64,

    #[serde(with = "string_to_float_optional")]
    pub avg_price: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub position_value: Option<f64>,

    pub trade_mode: i32,

    pub position_status: String,

    pub auto_add_margin: i32,

    pub adl_rank_indicator: i32,

    #[serde(with = "string_to_float_optional")]
    pub leverage: Option<f64>,

    #[serde(with = "string_to_float")]
    pub position_balance: f64,

    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    #[serde(with = "string_to_float_optional")]
    pub liq_price: Option<f64>,

    pub bust_price: String,

    #[serde(rename = "positionMM", with = "string_to_float_optional")]
    pub position_mm: Option<f64>,

    #[serde(rename = "positionIM", with = "string_to_float_optional")]
    pub position_im: Option<f64>,

    pub tpsl_mode: String,

    pub take_profit: String,

    pub stop_loss: String,

    pub trailing_stop: String,

    #[serde(with = "string_to_float_optional")]
    pub unrealised_pnl: Option<f64>,

    #[serde(with = "string_to_float_optional")]
    pub cum_realised_pnl: Option<f64>,

    pub seq: u64,

    pub is_reduce_only: bool,

    #[serde(with = "string_to_u64_optional")]
    pub mmr_sys_updated_time: Option<u64>,

    #[serde(with = "string_to_u64_optional")]
    pub leverage_sys_updated_time: Option<u64>,

    #[serde(with = "string_to_u64")]
    pub created_time: u64,

    #[serde(with = "string_to_u64")]
    pub updated_time: u64,
}
#[cfg(test)]
mod test_decode_position_info {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"
             {
                "positionIdx": 0,
                "riskId": 1,
                "riskLimitValue": "150",
                "symbol": "BTCUSD",
                "side": "Sell",
                "size": "300",
                "avgPrice": "27464.50441675",
                "positionValue": "0.01092319",
                "tradeMode": 0,
                "positionStatus": "Normal",
                "autoAddMargin": 1,
                "adlRankIndicator": 2,
                "leverage": "10",
                "positionBalance": "0.00139186",
                "markPrice": "28224.50",
                "liqPrice": "",
                "bustPrice": "999999.00",
                "positionMM": "0.0000015",
                "positionIM": "0.00010923",
                "tpslMode": "Full",
                "takeProfit": "0.00",
                "stopLoss": "0.00",
                "trailingStop": "0.00",
                "unrealisedPnl": "-0.00029413",
                "curRealisedPnl": "0.00013123",
                "cumRealisedPnl": "-0.00096902",
                "seq": 5723621632,
                "isReduceOnly": false,
                "mmrSysUpdatedTime": "1676538056444",
                "leverageSysUpdatedTime": "1676538056333",
                "sessionAvgPrice": "",
                "createdTime": "1676538056258",
                "updatedTime": "1697673600012"
            }
        "#;
        let result = serde_json::from_str::<PositionInfo>(json);
        let result = result.unwrap();
        assert_eq!(result.cum_realised_pnl, Some(-0.00096902));
        assert_eq!(result.mmr_sys_updated_time, Some(1676538056444));
        assert_eq!(result.leverage_sys_updated_time, Some(1676538056333));
        assert_eq!(result.created_time, 1676538056258);
        assert_eq!(result.updated_time, 1697673600012);
    }
}

#[derive(Clone, Default)]
pub struct LeverageRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub leverage: i8,
}

impl<'a> LeverageRequest<'a> {
    pub fn new(category: Category, symbol: &'a str, leverage: i8) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            leverage,
        }
    }
    pub fn default() -> LeverageRequest<'a> {
        LeverageRequest::new(Category::Linear, "BTCUSDT", 10)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeverageResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Default, Clone)]
pub struct ChangeMarginRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub trade_mode: i8,
    pub leverage: i8,
}

impl<'a> ChangeMarginRequest<'a> {
    pub fn new(category: Category, symbol: &'a str, trade_mode: i8, leverage: i8) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            trade_mode: match trade_mode {
                1 => 1,
                0 => 0,
                _ => 0,
            },
            leverage,
        }
    }
    pub fn default() -> ChangeMarginRequest<'a> {
        ChangeMarginRequest::new(Category::Linear, "BTCUSDT", 0, 10)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct MarginModeRequest<'a> {
    pub category: Category,
    pub mode: i8,
    pub symbol: Option<Cow<'a, str>>,
    pub coin: Option<Cow<'a, str>>,
}

impl<'a> MarginModeRequest<'a> {
    pub fn new(
        category: Category,
        mode: i8,
        symbol: Option<&'a str>,
        coin: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            mode,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            coin: coin.map(|s| Cow::Borrowed(s)),
        }
    }
    pub fn default() -> MarginModeRequest<'a> {
        MarginModeRequest::new(Category::Linear, 1, None, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct SetRiskLimit<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub risk_id: i8,
    pub position_idx: Option<i32>,
}

impl<'a> SetRiskLimit<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        risk_id: i8,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            risk_id,
            position_idx,
        }
    }
    pub fn default() -> SetRiskLimit<'a> {
        SetRiskLimit::new(Category::Linear, "BTCUSDT", 1, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: SetRiskLimitResult,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty, // Assuming retExtInfo is a JSON value as per provided JSON
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetRiskLimitResult {
    #[serde(rename = "riskId")]
    pub risk_id: i32,
    #[serde(rename = "riskLimitValue", with = "string_to_u64")]
    pub risk_limit_value: u64,
    pub category: String,
}

#[derive(Clone, Default)]
pub struct TradingStopRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<Cow<'a, str>>,
    pub sl_trigger_by: Option<Cow<'a, str>>,
    pub tpsl_mode: Option<Cow<'a, str>>,
    pub tp_order_type: Option<OrderType>,
    pub sl_order_type: Option<OrderType>,
    pub tp_size: Option<f64>,
    pub sl_size: Option<f64>,
    pub tp_limit_price: Option<f64>,
    pub sl_limit_price: Option<f64>,
    pub position_idx: i32,
}

impl<'a> TradingStopRequest<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        take_profit: Option<f64>,
        stop_loss: Option<f64>,
        tp_trigger_by: Option<&'a str>,
        sl_trigger_by: Option<&'a str>,
        tpsl_mode: Option<&'a str>,
        tp_order_type: Option<OrderType>,
        sl_order_type: Option<OrderType>,
        tp_size: Option<f64>,
        sl_size: Option<f64>,
        tp_limit_price: Option<f64>,
        sl_limit_price: Option<f64>,
        position_idx: i32,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            take_profit,
            stop_loss,
            tp_trigger_by: tp_trigger_by.map(|s| Cow::Borrowed(s)),
            sl_trigger_by: sl_trigger_by.map(|s| Cow::Borrowed(s)),
            tpsl_mode: tpsl_mode.map(|s| Cow::Borrowed(s)),
            tp_order_type,
            sl_order_type,
            tp_size,
            sl_size,
            tp_limit_price,
            sl_limit_price,
            position_idx,
        }
    }

    pub fn default() -> TradingStopRequest<'a> {
        TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            1,
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradingStopResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct AddMarginRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub auto_add: bool,
    pub position_idx: Option<i32>,
}

impl<'a> AddMarginRequest<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        auto_add: bool,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            auto_add,
            position_idx,
        }
    }
    pub fn default() -> AddMarginRequest<'a> {
        AddMarginRequest::new(Category::Linear, "BTCUSDT", false, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Empty, // Assuming result is an empty struct as per provided JSON
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Clone, Default)]
pub struct AddReduceMarginRequest<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub margin: f64,
    pub position_idx: Option<i32>,
}

impl<'a> AddReduceMarginRequest<'a> {
    pub fn new(
        category: Category,
        symbol: &'a str,
        margin: f64,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            margin,
            position_idx,
        }
    }
    pub fn default() -> AddReduceMarginRequest<'a> {
        AddReduceMarginRequest::new(Category::Linear, "BTCUSDT", 1.0, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddReduceMarginResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: AddReduceMarginResult,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty, // Assuming retExtInfo is an empty struct as per provided JSON
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddReduceMarginResult {
    pub category: String,
    pub symbol: String,
    pub position_idx: i32,
    #[serde(rename = "riskId")]
    pub risk_id: i32,
    #[serde(rename = "riskLimitValue")]
    pub risk_limit_value: String,
    pub size: String,
    #[serde(rename = "positionValue")]
    pub position_value: String,
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "liqPrice")]
    pub liq_price: String,
    #[serde(rename = "bustPrice")]
    pub bust_price: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    pub leverage: String,
    #[serde(rename = "autoAddMargin")]
    pub auto_add_margin: i32,
    #[serde(rename = "positionStatus")]
    pub position_status: String,
    #[serde(rename = "positionIM")]
    pub position_im: String,
    #[serde(rename = "positionMM")]
    pub position_mm: String,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,
    #[serde(rename = "cumRealisedPnl")]
    pub cum_realised_pnl: String,
    #[serde(rename = "stopLoss")]
    pub stop_loss: String,
    #[serde(rename = "takeProfit")]
    pub take_profit: String,
    #[serde(rename = "trailingStop")]
    pub trailing_stop: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
}

#[derive(Clone, Default)]
pub struct ClosedPnlRequest<'a> {
    pub category: Category,
    pub symbol: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
}

impl<'a> ClosedPnlRequest<'a> {
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            start_time,
            end_time,
            limit,
        }
    }
    pub fn default() -> ClosedPnlRequest<'a> {
        ClosedPnlRequest::new(Category::Linear, None, None, None, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: ClosedPnlResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    pub category: String,
    pub list: Vec<ClosedPnlItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlItem {
    pub symbol: String,
    pub order_type: String,
    pub leverage: String,
    pub updated_time: String,
    pub side: String,
    pub order_id: String,
    #[serde(with = "string_to_float")]
    pub closed_pnl: f64,
    #[serde(rename = "avgEntryPrice", with = "string_to_float")]
    pub avg_entry_price: f64,
    pub qty: String,
    #[serde(with = "string_to_float")]
    pub cum_entry_value: f64,
    pub created_time: String,
    #[serde(with = "string_to_float")]
    pub order_price: f64,
    pub closed_size: String,
    #[serde(rename = "avgExitPrice", with = "string_to_float")]
    pub avg_exit_price: f64,
    pub exec_type: String,
    pub fill_count: String,
    #[serde(with = "string_to_float")]
    pub cum_exit_value: f64,
}

#[derive(Clone, Default, Serialize)]
pub struct MovePositionRequest<'a> {
    pub from_uid: u64,
    pub to_uid: u64,
    pub list: Vec<PositionItem<'a>>,
}

#[derive(Clone, Default, Serialize)]
pub struct PositionItem<'a> {
    pub category: Category,
    pub symbol: Cow<'a, str>,
    pub price: f64,
    pub side: Side,
    pub qty: f64,
}

impl<'a> MovePositionRequest<'a> {
    pub fn new(from_uid: u64, to_uid: u64, list: Vec<PositionItem<'a>>) -> Self {
        Self {
            from_uid,
            to_uid,
            list,
        }
    }
    pub fn default() -> MovePositionRequest<'a> {
        MovePositionRequest::new(0, 0, vec![])
    }
}
impl<'a> PositionItem<'a> {
    pub fn new(category: Category, symbol: &'a str, price: f64, side: Side, qty: f64) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            price,
            side,
            qty,
        }
    }
    pub fn default() -> PositionItem<'a> {
        PositionItem::new(Category::Linear, "BTCUSDT", 0.0, Side::Buy, 0.0)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MovePositionResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: MovePositionResult,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MovePositionResult {
    pub block_trade_id: String,
    pub status: String,
    pub reject_party: String,
}

#[derive(Serialize, Clone, Default)]
pub struct MoveHistoryRequest<'a> {
    pub category: Option<Category>,
    pub symbol: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub status: Option<Cow<'a, str>>,
    pub block_trade_id: Option<Cow<'a, str>>,
    pub limit: Option<Cow<'a, str>>,
}

impl<'a> MoveHistoryRequest<'a> {
    pub fn new(
        category: Option<Category>,
        symbol: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        status: Option<&'a str>,
        block_trade_id: Option<&'a str>,
        limit: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(|s| Cow::Borrowed(s)),
            start_time,
            end_time,
            status: status.map(|s| Cow::Borrowed(s)),
            block_trade_id: block_trade_id.map(|s| Cow::Borrowed(s)),
            limit: limit.map(|s| Cow::Borrowed(s)),
        }
    }
    pub fn default() -> MoveHistoryRequest<'a> {
        MoveHistoryRequest::new(None, None, None, None, None, None, None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveHistoryResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i16,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: MoveHistoryResult,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveHistoryResult {
    pub list: Vec<MoveHistoryEntry>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveHistoryEntry {
    #[serde(rename = "blockTradeId")]
    pub block_trade_id: String,
    pub category: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub symbol: String,
    pub side: String,
    pub price: String,
    pub qty: String,
    #[serde(rename = "execFee")]
    pub exec_fee: String,
    pub status: String,
    #[serde(rename = "execId")]
    pub exec_id: String,
    #[serde(rename = "resultCode")]
    pub result_code: i16,
    #[serde(rename = "resultMessage")]
    pub result_message: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
    #[serde(rename = "rejectParty")]
    pub reject_party: String,
}

// = = = = = = = = =  ==  = = == = = == =  = = = = = = = = = = = = = = = =
//
//  ACCOUNT STRUCTS AND RESPONSES
//
// = = = = = = = = = = = = = = = = = = ==  = = = = ==  = = == = =  = = = =

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: WalletList,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WalletList {
    pub list: Vec<WalletData>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UTAResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: UTAUpdateStatus,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UTAUpdateStatus {
    #[serde(rename = "unifiedUpdateStatus")]
    pub unified_update_status: String,
    #[serde(rename = "unifiedUpdateMsg")]
    pub unified_update_msg: UnifiedUpdateMsg,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UnifiedUpdateMsg {
    pub msg: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BorrowHistoryRequest<'a> {
    pub coin: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<Cow<'a, str>>,
}

impl<'a> BorrowHistoryRequest<'a> {
    pub fn new(
        coin: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<&'a str>,
    ) -> Self {
        Self {
            coin: coin.map(|s| Cow::Borrowed(s)),
            start_time,
            end_time,
            limit: limit.map(|s| Cow::Borrowed(s)),
        }
    }
    pub fn default() -> BorrowHistoryRequest<'a> {
        BorrowHistoryRequest::new(None, None, None, None)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: BorrowHistory,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistory {
    pub next_page_cursor: String,
    pub rows: Vec<BorrowHistoryEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryEntry {
    #[serde(rename = "borrowAmount")]
    pub borrow_amount: String,
    #[serde(rename = "costExemption")]
    pub cost_exemption: String,
    #[serde(rename = "freeBorrowedAmount")]
    pub free_borrowed_amount: String,
    #[serde(rename = "createdTime")]
    pub created_time: u64,
    #[serde(rename = "InterestBearingBorrowSize")]
    pub interest_bearing_borrow_size: String,
    pub currency: String,
    #[serde(rename = "unrealisedLoss")]
    pub unrealised_loss: String,
    #[serde(rename = "hourlyBorrowRate")]
    pub hourly_borrow_rate: String,
    #[serde(rename = "borrowCost")]
    pub borrow_cost: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepayLiabilityResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: LiabilityQty,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiabilityQty {
    pub list: Vec<LiabilityQtyData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiabilityQtyData {
    pub coin: String,
    pub repayment_qty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetCollateralCoinResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Empty,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralCoinResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: SwitchList,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchList {
    pub list: Vec<SwitchListData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchListData {
    pub coin: String,
    pub collateral_switch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: CollateralInfoList,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoList {
    pub list: Vec<CollateralInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfo {
    #[serde(rename = "availableToBorrow")]
    pub available_to_borrow: String,
    #[serde(rename = "freeBorrowingAmount")]
    pub free_borrowing_amount: String,
    #[serde(rename = "freeBorrowAmount")]
    pub free_borrow_amount: String,
    #[serde(rename = "maxBorrowingAmount")]
    pub max_borrowing_amount: String,
    #[serde(rename = "hourlyBorrowRate")]
    pub hourly_borrow_rate: String,
    #[serde(rename = "borrowUsageRate")]
    pub borrow_usage_rate: String,
    #[serde(rename = "collateralSwitch")]
    pub collateral_switch: bool,
    #[serde(rename = "borrowAmount")]
    pub borrow_amount: String,
    #[serde(rename = "borrowable")]
    pub borrowable: bool,
    pub currency: String,
    #[serde(rename = "marginCollateral")]
    pub margin_collateral: bool,
    #[serde(rename = "freeBorrowingLimit")]
    pub free_borrowing_limit: String,
    #[serde(rename = "collateralRatio")]
    pub collateral_ratio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: FeeRateList,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateList {
    pub list: Vec<FeeRate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeeRate {
    pub symbol: String,
    pub maker_fee_rate: String,
    pub taker_fee_rate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: AccountInfo,
    #[serde(default)]
    pub ret_ext_info: Empty,
    #[serde(default)]
    pub time: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub margin_mode: String,
    pub updated_time: String,
    pub unified_margin_status: i8,
    pub dcp_status: String,
    pub time_window: i32,
    pub smp_group: i8,
    pub is_master_trader: bool,
    pub spot_hedging_status: String,
}

#[derive(Clone, Default)]
pub struct TransactionLogRequest<'a> {
    pub account_type: Option<Cow<'a, str>>,
    pub category: Option<Category>,
    pub currency: Option<Cow<'a, str>>,
    pub base_coin: Option<Cow<'a, str>>,
    pub log_type: Option<Cow<'a, str>>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
}

impl<'a> TransactionLogRequest<'a> {
    pub fn new(
        account_type: Option<&'a str>,
        category: Option<Category>,
        currency: Option<&'a str>,
        base_coin: Option<&'a str>,
        log_type: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Self {
        Self {
            account_type: account_type.map(|s| Cow::Borrowed(s)),
            category,
            currency: currency.map(|s| Cow::Borrowed(s)),
            base_coin: base_coin.map(|s| Cow::Borrowed(s)),
            log_type: log_type.map(|s| Cow::Borrowed(s)),
            start_time,
            end_time,
            limit,
        }
    }
    pub fn default() -> Self {
        Self::new(None, None, None, None, None, None, None, None)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogEntry {
    pub id: String,
    pub symbol: String,
    pub side: String,
    pub funding: Option<String>,
    pub order_link_id: Option<String>,
    pub order_id: String,
    pub fee: String,
    pub change: String,
    pub cash_flow: String,
    pub transaction_time: String,
    pub type_field: String,
    #[serde(rename = "feeRate")]
    pub fee_rate: String,
    pub bonus_change: Option<String>,
    pub size: String,
    pub qty: String,
    pub cash_balance: String,
    pub currency: String,
    pub category: String,
    pub trade_price: String,
    pub trade_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogResult {
    pub next_page_cursor: String,
    pub list: Vec<TransactionLogEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: TransactionLogResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmpResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: SmpResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SmpResult {
    pub smp_group: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: MarginModeResult,
    pub ret_ext_info: Empty,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResult {
    pub reason: Vec<ReasonObject>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReasonObject {
    pub reason_code: String,
    pub reason_msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotHedgingResponse {
    pub ret_code: i32,
    pub ret_msg: String,
}

// = = = = = = = = = = = = ==  = == = =  =  = = = = ==
// HEADER STRUCT FOR TRADESTREM RESPONSE
// = = = = = = = = = = = = ==  = == = =  =  = = = = ==

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    #[serde(rename = "X-Bapi-Limit")]
    pub x_bapi_limit: String,
    #[serde(rename = "X-Bapi-Limit-Status")]
    pub x_bapi_limit_status: String,
    #[serde(rename = "X-Bapi-Limit-Reset-Timestamp")]
    pub x_bapi_limit_reset_timestamp: String,
    #[serde(rename = "Traceid")]
    pub traceid: String,
    #[serde(rename = "Timenow")]
    pub timenow: String,
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
//
// WEBSOCKET STRUCTS AND RESPONSES
//
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

#[derive(Clone, Debug, Default)]
pub struct Subscription<'a> {
    pub op: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Subscription<'a> {
    pub fn new(op: &'a str, args: Vec<&'a str>) -> Self {
        Self { op, args }
    }
    pub fn default() -> Subscription<'a> {
        Subscription::new("subscribe", vec![])
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WebsocketEvents {
    OrderBookEvent(OrderBookUpdate),
    TradeEvent(TradeUpdate),
    TickerEvent(WsTicker),
    LiquidationEvent(Liquidation),
    KlineEvent(WsKline),
    PositionEvent(PositionEvent),
    ExecutionEvent(Execution),
    OrderEvent(OrderEvent),
    Wallet(WalletEvent),
    TradeStream(TradeStreamEvent),
    FastExecEvent(FastExecution),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Tickers {
    Linear(LinearTickerData),
    Spot(SpotTickerData),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PongResponse {
    PublicPong(PongData),
    PrivatePong(PongData),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PongData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    pub ret_msg: String,
    pub conn_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    pub op: String,
}

unsafe impl Send for PongData {}
unsafe impl Sync for PongData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeStreamEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    pub ret_code: i32,
    pub ret_msg: String,
    pub op: String,
    pub data: OrderStatus,
    pub header: Header,
    pub conn_id: String,
}

unsafe impl Send for TradeStreamEvent {}
unsafe impl Sync for TradeStreamEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookUpdate {
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub data: WsOrderBook,
    pub cts: u64,
}

unsafe impl Send for OrderBookUpdate {}
unsafe impl Sync for OrderBookUpdate {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsOrderBook {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,
    #[serde(rename = "u")]
    pub update_id: u64,
    pub seq: u64,
}

unsafe impl Send for WsOrderBook {}
unsafe impl Sync for WsOrderBook {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeUpdate {
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub data: Vec<WsTrade>,
}

unsafe impl Send for TradeUpdate {}
unsafe impl Sync for TradeUpdate {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsTrade {
    #[serde(rename = "T")]
    pub timestamp: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "v", with = "string_to_float")]
    pub volume: f64,
    #[serde(rename = "p", with = "string_to_float")]
    pub price: f64,
    #[serde(rename = "L")]
    pub tick_direction: String,
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "BT")]
    pub buyer_is_maker: bool,
}

unsafe impl Send for WsTrade {}
unsafe impl Sync for WsTrade {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsTicker {
    pub topic: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: Tickers,
    pub cs: u64,
    pub ts: u64,
}

unsafe impl Send for WsTicker {}
unsafe impl Sync for WsTicker {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinearTickerData {
    pub symbol: String,
    #[serde(rename = "tickDirection")]
    pub tick_direction: String,
    #[serde(rename = "price24hPcnt")]
    pub price_24h_pcnt: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "prevPrice24h")]
    pub prev_price_24h: String,
    #[serde(rename = "highPrice24h")]
    pub high_price_24h: String,
    #[serde(rename = "lowPrice24h")]
    pub low_price_24h: String,
    #[serde(rename = "prevPrice1h")]
    pub prev_price_1h: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "indexPrice")]
    pub index_price: String,
    #[serde(rename = "openInterest")]
    pub open_interest: String,
    #[serde(rename = "openInterestValue")]
    pub open_interest_value: String,
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: String,
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "bid1Price")]
    pub bid_price: String,
    #[serde(rename = "bid1Size")]
    pub bid_size: String,
    #[serde(rename = "ask1Price")]
    pub ask_price: String,
    #[serde(rename = "ask1Size")]
    pub ask_size: String,
}

unsafe impl Send for LinearTickerData {}
unsafe impl Sync for LinearTickerData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotTickerData {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "highPrice24h")]
    pub high_price_24h: String,
    #[serde(rename = "lowPrice24h")]
    pub low_price_24h: String,
    #[serde(rename = "prevPrice24h")]
    pub prev_price_24h: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    #[serde(rename = "price24hPcnt")]
    pub price_24h_pcnt: String,
    #[serde(rename = "usdIndexPrice")]
    pub usd_index_price: String,
}

unsafe impl Send for SpotTickerData {}
unsafe impl Sync for SpotTickerData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Liquidation {
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "ts")]
    pub ts: u64,
    #[serde(rename = "data")]
    pub data: LiquidationData,
}

unsafe impl Send for Liquidation {}
unsafe impl Sync for Liquidation {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidationData {
    #[serde(rename = "updatedTime")]
    pub updated_time: u64,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(with = "string_to_float")]
    pub size: f64,
    #[serde(with = "string_to_float")]
    pub price: f64,
}

unsafe impl Send for LiquidationData {}
unsafe impl Sync for LiquidationData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsKline {
    pub topic: String,
    pub data: Vec<KlineData>,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub event_type: String,
}

unsafe impl Send for WsKline {}
unsafe impl Sync for WsKline {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KlineData {
    pub start: u64,
    pub end: u64,
    pub interval: String,
    pub open: String,
    pub close: String,
    pub high: String,
    pub low: String,
    pub volume: String,
    pub turnover: String,
    pub confirm: bool,
    pub timestamp: u64,
}

unsafe impl Send for KlineData {}
unsafe impl Sync for KlineData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionEvent {
    pub id: String,
    pub topic: String,
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    pub data: Vec<PositionData>,
}

unsafe impl Send for PositionEvent {}
unsafe impl Sync for PositionEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionData {
    #[serde(rename = "positionIdx")]
    pub position_idx: u8,
    #[serde(rename = "tradeMode")]
    pub trade_mode: u8,
    #[serde(rename = "riskId")]
    pub risk_id: u8,
    #[serde(rename = "riskLimitValue")]
    pub risk_limit_value: String,
    pub symbol: String,
    pub side: String,
    pub size: String,
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    pub leverage: String,
    #[serde(rename = "positionValue")]
    pub position_value: String,
    #[serde(rename = "positionBalance")]
    pub position_balance: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "positionIM")]
    pub position_im: String,
    #[serde(rename = "positionMM")]
    pub position_mm: String,
    #[serde(rename = "takeProfit")]
    pub take_profit: String,
    #[serde(rename = "stopLoss")]
    pub stop_loss: String,
    #[serde(rename = "trailingStop")]
    pub trailing_stop: String,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,
    #[serde(rename = "cumRealisedPnl")]
    pub cum_realised_pnl: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,
    #[serde(rename = "liqPrice")]
    pub liq_price: String,
    #[serde(rename = "bustPrice")]
    pub bust_price: String,
    pub category: String,
    #[serde(rename = "positionStatus")]
    pub position_status: String,
    #[serde(rename = "adlRankIndicator")]
    pub adl_rank_indicator: u8,
    #[serde(rename = "autoAddMargin")]
    pub auto_add_margin: u8,
    #[serde(rename = "leverageSysUpdatedTime")]
    pub leverage_sys_updated_time: String,
    #[serde(rename = "mmrSysUpdatedTime")]
    pub mmr_sys_updated_time: String,
    pub seq: u64,
    #[serde(rename = "isReduceOnly")]
    pub is_reduce_only: bool,
}

unsafe impl Send for PositionData {}
unsafe impl Sync for PositionData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Execution {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    #[serde(rename = "data")]
    pub data: Vec<ExecutionData>,
}

unsafe impl Send for Execution {}
unsafe impl Sync for Execution {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecutionData {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "execFee")]
    pub exec_fee: String,
    #[serde(rename = "execId")]
    pub exec_id: String,
    #[serde(rename = "execPrice")]
    pub exec_price: String,
    #[serde(rename = "execQty")]
    pub exec_qty: String,
    #[serde(rename = "execType")]
    pub exec_type: String,
    #[serde(rename = "execValue")]
    pub exec_value: String,
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
    #[serde(rename = "feeRate")]
    pub fee_rate: String,
    #[serde(rename = "tradeIv")]
    pub trade_iv: String,
    #[serde(rename = "markIv")]
    pub mark_iv: String,
    #[serde(rename = "blockTradeId")]
    pub block_trade_id: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "indexPrice")]
    pub index_price: String,
    #[serde(rename = "underlyingPrice")]
    pub underlying_price: String,
    #[serde(rename = "leavesQty")]
    pub leaves_qty: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
    #[serde(rename = "orderPrice")]
    pub order_price: String,
    #[serde(rename = "orderQty")]
    pub order_qty: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "execTime")]
    pub exec_time: String,
    #[serde(rename = "isLeverage")]
    pub is_leverage: String,
    #[serde(rename = "closedSize")]
    pub closed_size: String,
    #[serde(rename = "seq")]
    pub seq: u64,
}

unsafe impl Send for ExecutionData {}
unsafe impl Sync for ExecutionData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FastExecution {
    pub topic: String,
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    pub data: Vec<FastExecData>,
}

unsafe impl Send for FastExecution {}
unsafe impl Sync for FastExecution {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FastExecData {
    pub category: String,
    pub symbol: String,
    #[serde(rename = "execId")]
    pub exec_id: String,
    #[serde(rename = "execPrice")]
    pub exec_price: String,
    #[serde(rename = "execQty")]
    pub exec_qty: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
    pub side: String,
    #[serde(rename = "execTime")]
    pub exec_time: String,
    pub seq: u64,
}

unsafe impl Send for FastExecData {}
unsafe impl Sync for FastExecData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderData {
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub side: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "cancelType")]
    pub cancel_type: String,
    pub price: String,
    pub qty: String,
    #[serde(rename = "orderIv")]
    pub order_iv: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
    #[serde(rename = "lastPriceOnCreated")]
    pub last_price_on_created: String,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "leavesQty")]
    pub leaves_qty: String,
    #[serde(rename = "leavesValue")]
    pub leaves_value: String,
    #[serde(rename = "cumExecQty")]
    pub cum_exec_qty: String,
    #[serde(rename = "cumExecValue")]
    pub cum_exec_value: String,
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "blockTradeId")]
    pub block_trade_id: String,
    #[serde(rename = "positionIdx")]
    pub position_idx: u8,
    #[serde(rename = "cumExecFee")]
    pub cum_exec_fee: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
    #[serde(rename = "rejectReason")]
    pub reject_reason: String,
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,
    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,
    #[serde(rename = "triggerPrice")]
    pub trigger_price: String,
    #[serde(rename = "takeProfit")]
    pub take_profit: String,
    #[serde(rename = "stopLoss")]
    pub stop_loss: String,
    #[serde(rename = "tpTriggerBy")]
    pub tp_trigger_by: String,
    #[serde(rename = "slTriggerBy")]
    pub sl_trigger_by: String,
    #[serde(rename = "tpLimitPrice")]
    pub tp_limit_price: String,
    #[serde(rename = "slLimitPrice")]
    pub sl_limit_price: String,
    #[serde(rename = "triggerDirection")]
    pub trigger_direction: u8,
    #[serde(rename = "triggerBy")]
    pub trigger_by: String,
    #[serde(rename = "closeOnTrigger")]
    pub close_on_trigger: bool,
    pub category: String,
    #[serde(rename = "placeType")]
    pub place_type: String,
    #[serde(rename = "smpType")]
    pub smp_type: String,
    #[serde(rename = "smpGroup")]
    pub smp_group: u8,
    #[serde(rename = "smpOrderId")]
    pub smp_order_id: String,
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,
}

unsafe impl Send for OrderData {}
unsafe impl Sync for OrderData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderEvent {
    pub id: String,
    pub topic: String,
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    pub data: Vec<OrderData>,
}

unsafe impl Send for OrderEvent {}
unsafe impl Sync for OrderEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletEvent {
    pub id: String,
    pub topic: String,
    #[serde(rename = "creationTime")]
    pub creation_time: u64,
    pub data: Vec<WalletData>,
}
unsafe impl Send for WalletEvent {}
unsafe impl Sync for WalletEvent {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletData {
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: String,
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: String,
    #[serde(rename = "totalEquity")]
    pub total_equity: String,
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    #[serde(rename = "totalAvailableBalance")]
    pub total_available_balance: String,
    #[serde(rename = "totalPerpUPL")]
    pub total_perp_upl: String,
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: String,
    #[serde(rename = "totalMaintenanceMargin")]
    pub total_maintenance_margin: String,
    #[serde(rename = "coin")]
    pub coin: Vec<CoinData>,
    #[serde(rename = "accountLTV")]
    pub account_ltv: String,
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}
unsafe impl Send for WalletData {}
unsafe impl Sync for WalletData {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinData {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "equity")]
    pub equity: String,
    #[serde(rename = "usdValue")]
    pub usd_value: String,
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,
    #[serde(rename = "availableToWithdraw")]
    pub available_to_withdraw: String,
    #[serde(rename = "availableToBorrow")]
    pub available_to_borrow: String,
    #[serde(rename = "borrowAmount")]
    pub borrow_amount: String,
    #[serde(rename = "accruedInterest")]
    pub accrued_interest: String,
    #[serde(rename = "totalOrderIM")]
    pub total_order_im: String,
    #[serde(rename = "totalPositionIM")]
    pub total_position_im: String,
    #[serde(rename = "totalPositionMM")]
    pub total_position_mm: String,
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,
    #[serde(rename = "cumRealisedPnl")]
    pub cum_realised_pnl: String,
    pub bonus: String,
    #[serde(rename = "collateralSwitch")]
    pub collateral_switch: bool,
    #[serde(rename = "marginCollateral")]
    pub margin_collateral: bool,
    #[serde(rename = "locked")]
    pub locked: String,
    #[serde(rename = "spotHedgingQty")]
    pub spot_hedging_qty: String,
}

unsafe impl Send for CoinData {}
unsafe impl Sync for CoinData {}

mod string_to_u64 {
    use std::default;

    use serde::{self, Deserialize, Deserializer, Serializer};

    // Serialize a u64 as a string.
    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_string();
        serializer.serialize_str(&s)
    }

    // Deserialize a string to a u64.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

pub mod string_to_float {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // Serialize a u64 as a string.
    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_string();
        serializer.serialize_str(&s)
    }

    // Deserialize a string as an f64.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<f64>().map_err(serde::de::Error::custom)
    }
}

mod string_to_u64_optional {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    // Serialization: Convert Option<u64> to string
    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_str(""),
        }
    }

    // Deserialization: Parse string to Option<u64>, return None for empty string
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None) // Return None for empty string
        } else {
            u64::from_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}
