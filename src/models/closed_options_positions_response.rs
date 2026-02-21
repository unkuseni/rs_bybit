use crate::prelude::*;

/// Represents a single closed options position item.
///
/// Contains detailed information about a closed options position, including entry/exit prices,
/// fees, P&L, and timing. Bots use this to analyze options trading performance, calculate
/// profitability metrics, and audit trading history.
///
/// # Bybit API Reference
/// According to the Bybit V5 API documentation:
/// - Fee and price are displayed with trailing zeroes up to 8 decimal places.
/// - Positions are sorted by `closeTime` in descending order.
/// - Only supports querying closed options positions in the last 6 months.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClosedOptionsPositionItem {
    /// The options symbol name (e.g., "BTC-12JUN25-104019-C-USDT").
    ///
    /// Identifies the specific options contract. Bots use this to track performance
    /// by symbol and analyze specific options strategies.
    pub symbol: String,

    /// The position side ("Buy" or "Sell").
    ///
    /// Indicates whether the position was long (Buy) or short (Sell) the option.
    /// Bots use this to calculate directional exposure and analyze strategy performance.
    pub side: Side,

    /// The total open fee paid for the position.
    ///
    /// The cumulative fee paid when opening the position. Bots use this to calculate
    /// net profitability and optimize for fee efficiency.
    #[serde(with = "string_to_float")]
    pub total_open_fee: f64,

    /// The delivery fee (if applicable).
    ///
    /// The fee charged for options delivery at expiration. Bots use this to account
    /// for expiration costs in options strategies.
    #[serde(with = "string_to_float")]
    pub delivery_fee: f64,

    /// The total close fee paid for the position.
    ///
    /// The cumulative fee paid when closing the position. Bots use this to calculate
    /// total transaction costs and net P&L.
    #[serde(with = "string_to_float")]
    pub total_close_fee: f64,

    /// The position quantity.
    ///
    /// The number of options contracts in the position. Bots use this to calculate
    /// position size and exposure.
    #[serde(with = "string_to_float")]
    pub qty: f64,

    /// The timestamp when the position was closed (in milliseconds).
    ///
    /// Indicates when the position was closed. Bots use this for time-series analysis
    /// and to correlate position closures with market events.
    #[serde(with = "string_to_u64")]
    pub close_time: u64,

    /// The average exit price.
    ///
    /// The average price at which the position was closed. Bots use this to calculate
    /// exit efficiency and compare against target exit prices.
    #[serde(with = "string_to_float")]
    pub avg_exit_price: f64,

    /// The delivery price (if applicable).
    ///
    /// The settlement price at options expiration. Bots use this to analyze
    /// expiration outcomes for options held to maturity.
    #[serde(with = "string_to_float")]
    pub delivery_price: f64,

    /// The timestamp when the position was opened (in milliseconds).
    ///
    /// Indicates when the position was initially opened. Bots use this to calculate
    /// position duration and analyze holding period returns.
    #[serde(with = "string_to_u64")]
    pub open_time: u64,

    /// The average entry price.
    ///
    /// The average price at which the position was opened. Bots use this to calculate
    /// entry efficiency and compare against target entry prices.
    #[serde(with = "string_to_float")]
    pub avg_entry_price: f64,

    /// The total profit and loss for the position.
    ///
    /// The net P&L including all fees. Bots use this as the primary performance metric
    /// for options trading strategies.
    #[serde(with = "string_to_float")]
    pub total_pnl: f64,
}

/// Represents the response from the `/v5/position/get-closed-positions` endpoint.
///
/// Contains a paginated list of closed options positions with metadata for continued
/// pagination. Bots use this to retrieve historical options trading data for analysis,
/// reporting, and strategy optimization.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClosedOptionsPositionsResponse {
    /// The pagination cursor for retrieving the next page of results.
    ///
    /// Use this token in subsequent requests to retrieve the next page of closed positions.
    /// Returns an empty string if there are no more results. Bots use this for efficient
    /// pagination through large result sets.
    pub next_page_cursor: String,

    /// The product category (always "option" for this endpoint).
    ///
    /// Confirms the instrument type. Bots should validate this matches the request category.
    pub category: String,

    /// The list of closed options positions.
    ///
    /// Contains detailed information for each closed position, sorted by `closeTime`
    /// in descending order. Bots iterate through this list to analyze trading history.
    pub list: Vec<ClosedOptionsPositionItem>,
}

/// Type alias for the API response containing closed options positions.
///
/// Standardized response type that includes retCode, retMsg, result, retExtInfo, and time.
/// Bots use this to handle both successful responses and errors uniformly.
pub type ClosedOptionsPositionsResult = BybitApiResponse<ClosedOptionsPositionsResponse>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_closed_options_position_item() {
        let json_data = json!({
            "symbol": "BTC-12JUN25-104019-C-USDT",
            "side": "Sell",
            "totalOpenFee": "0.94506647",
            "deliveryFee": "0.32184533",
            "totalCloseFee": "0.00000000",
            "qty": "0.02",
            "closeTime": "1749726002161",
            "avgExitPrice": "107281.77405000",
            "deliveryPrice": "107281.77405031",
            "openTime": "1749722990063",
            "avgEntryPrice": "3371.50000000",
            "totalPnl": "0.90760719"
        });

        let item: ClosedOptionsPositionItem =
            serde_json::from_value(json_data).expect("Failed to deserialize");

        assert_eq!(item.symbol, "BTC-12JUN25-104019-C-USDT");
        assert_eq!(item.side, Side::Sell);
        assert_eq!(item.total_open_fee, 0.94506647);
        assert_eq!(item.delivery_fee, 0.32184533);
        assert_eq!(item.total_close_fee, 0.0);
        assert_eq!(item.qty, 0.02);
        assert_eq!(item.close_time, 1749726002161);
        assert_eq!(item.avg_exit_price, 107281.77405);
        assert_eq!(item.delivery_price, 107281.77405031);
        assert_eq!(item.open_time, 1749722990063);
        assert_eq!(item.avg_entry_price, 3371.5);
        assert_eq!(item.total_pnl, 0.90760719);
    }

    #[test]
    fn test_deserialize_closed_options_positions_response() {
        let json_data = json!({
            "nextPageCursor": "1749726002161%3A0%2C1749715220240%3A1",
            "category": "option",
            "list": [
                {
                    "symbol": "BTC-12JUN25-104019-C-USDT",
                    "side": "Sell",
                    "totalOpenFee": "0.94506647",
                    "deliveryFee": "0.32184533",
                    "totalCloseFee": "0.00000000",
                    "qty": "0.02",
                    "closeTime": "1749726002161",
                    "avgExitPrice": "107281.77405000",
                    "deliveryPrice": "107281.77405031",
                    "openTime": "1749722990063",
                    "avgEntryPrice": "3371.50000000",
                    "totalPnl": "0.90760719"
                },
                {
                    "symbol": "BTC-12JUN25-104000-C-USDT",
                    "side": "Buy",
                    "totalOpenFee": "0.86379999",
                    "deliveryFee": "0.32287622",
                    "totalCloseFee": "0.00000000",
                    "qty": "0.02",
                    "closeTime": "1749715220240",
                    "avgExitPrice": "107625.40470150",
                    "deliveryPrice": "107625.40470159",
                    "openTime": "1749710568608",
                    "avgEntryPrice": "3946.50000000",
                    "totalPnl": "-7.60858218"
                }
            ]
        });

        let response: ClosedOptionsPositionsResponse =
            serde_json::from_value(json_data).expect("Failed to deserialize");

        assert_eq!(
            response.next_page_cursor,
            "1749726002161%3A0%2C1749715220240%3A1"
        );
        assert_eq!(response.category, "option");
        assert_eq!(response.list.len(), 2);

        let first_item = &response.list[0];
        assert_eq!(first_item.symbol, "BTC-12JUN25-104019-C-USDT");
        assert_eq!(first_item.side, Side::Sell);
        assert_eq!(first_item.total_pnl, 0.90760719);

        let second_item = &response.list[1];
        assert_eq!(second_item.symbol, "BTC-12JUN25-104000-C-USDT");
        assert_eq!(second_item.side, Side::Buy);
        assert_eq!(second_item.total_pnl, -7.60858218);
    }

    #[test]
    fn test_deserialize_closed_options_positions_result() {
        let json_data = json!({
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "nextPageCursor": "1749726002161%3A0%2C1749715220240%3A1",
                "category": "option",
                "list": [
                    {
                        "symbol": "BTC-12JUN25-104019-C-USDT",
                        "side": "Sell",
                        "totalOpenFee": "0.94506647",
                        "deliveryFee": "0.32184533",
                        "totalCloseFee": "0.00000000",
                        "qty": "0.02",
                        "closeTime": "1749726002161",
                        "avgExitPrice": "107281.77405000",
                        "deliveryPrice": "107281.77405031",
                        "openTime": "1749722990063",
                        "avgEntryPrice": "3371.50000000",
                        "totalPnl": "0.90760719"
                    }
                ]
            },
            "retExtInfo": {},
            "time": 1672284129
        });

        let result: ClosedOptionsPositionsResult =
            serde_json::from_value(json_data).expect("Failed to deserialize");

        assert_eq!(result.ret_code, 0);
        assert_eq!(result.ret_msg, "OK");
        assert_eq!(result.result.category, "option");
        assert_eq!(result.result.list.len(), 1);
        assert_eq!(result.time, 1672284129);
    }

    #[test]
    fn test_serialize_closed_options_position_item() {
        let item = ClosedOptionsPositionItem {
            symbol: "BTC-12JUN25-104019-C-USDT".to_string(),
            side: Side::Sell,
            total_open_fee: 0.94506647,
            delivery_fee: 0.32184533,
            total_close_fee: 0.0,
            qty: 0.02,
            close_time: 1749726002161,
            avg_exit_price: 107281.77405,
            delivery_price: 107281.77405031,
            open_time: 1749722990063,
            avg_entry_price: 3371.5,
            total_pnl: 0.90760719,
        };

        let json_string = serde_json::to_string(&item).expect("Failed to serialize");
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert_eq!(parsed["symbol"], "BTC-12JUN25-104019-C-USDT");
        assert_eq!(parsed["side"], "Sell");
        assert_eq!(parsed["totalOpenFee"], "0.94506647");
        assert_eq!(parsed["totalPnl"], "0.90760719");
    }
}
