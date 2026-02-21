use bybit::prelude::*;

use tokio::time::{Duration, Instant};

#[cfg(test)]
mod tests {
    use super::*;

    use tokio::test;

    #[test]
    async fn test_kline() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "ETHUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let klines = market.get_klines(request).await;
        if let Ok(data) = klines {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_mark_klines() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "ETHUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let mark_klines = market.get_mark_price_klines(request).await;
        if let Ok(data) = mark_klines {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_index_klines() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "ETHUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let index_klines = market.get_index_price_klines(request).await;
        if let Ok(data) = index_klines {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_premium_klines() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "ETHUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let premium_klines = market.get_premium_index_price_klines(request).await;
        if let Ok(data) = premium_klines {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_futures_instrument() {
        let market: MarketData = Bybit::new(None, None);
        let request = InstrumentRequest::new(Category::Linear, Some("ETHUSDT"), None, None, None);
        let instrument = market.get_instrument_info(request).await;
        if let Ok(data) = instrument {
            match data.result {
                InstrumentInfo::Futures(futures) => println!("{:#?}", futures.list[0]),
                _ => println!("not futures"),
            }
        }
    }

    #[test]
    async fn test_spot_instrument() {
        let market: MarketData = Bybit::new(None, None);
        let request = InstrumentRequest::new(Category::Spot, Some("ETHUSDT"), None, None, None);
        let instrument = market.get_instrument_info(request).await;
        if let Ok(data) = instrument {
            match data.result {
                InstrumentInfo::Spot(spot) => println!("{:#?}", spot.list[0]),
                _ => println!("not spot"),
            }
        }
    }

    #[test]
    async fn test_market() {
        let market: MarketData = Bybit::new(None, None);
        let five_minutes = Duration::from_secs(5 * 60);
        let request = OrderbookRequest::new("ETHUSDT", Category::Linear, Some(1));
        let start = Instant::now();
        while Instant::now() - start < five_minutes {
            let order_book = market.get_depth(request.clone()).await;
            if let Ok(data) = order_book {
                let order_book = data.result;
                let mid_price = (order_book.asks[0].price + order_book.bids[0].price) / 2.0;
                let imbalance = (order_book.bids[0].qty - order_book.asks[0].qty)
                    / (order_book.asks[0].qty + order_book.bids[0].qty);
                let fees = fee_percent(mid_price, 0.04);
                let spread = order_book.asks[0].price - order_book.bids[0].price;
                let arb = spread - fees;
                println!(
                    "{:#?} , Spread: {:.5} Arb: {} Imb: {:.4}",
                    order_book,
                    spread,
                    if arb > fee_percent(mid_price, 0.02) {
                        arb
                    } else {
                        0.0
                    },
                    imbalance
                );
            }
        }
    }

    fn fee_percent(value: f64, percent: f64) -> f64 {
        (percent / 100.0) * value
    }

    #[test]
    async fn test_futures_ticker() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "ETHUSDT";
        let futures_ticker = market.get_tickers(Some(symbol), Category::Linear).await;
        if let Ok(data) = futures_ticker {
            match &data.result.list[0] {
                TickerData::Futures(futures) => println!("{:#?}", futures),
                _ => println!("not futures"),
            }
        }
    }

    #[test]
    async fn test_spot_ticker() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "ETHUSDT";
        let spot_ticker = market.get_tickers(Some(symbol), Category::Spot).await;
        if let Ok(data) = spot_ticker {
            match &data.result.list[0].clone() {
                TickerData::Spot(spot) => println!("{:#?}", spot),
                _ => println!("not spot"),
            }
        }
    }

    #[test]
    async fn test_recent_trades() {
        let market: MarketData = Bybit::new(None, None);
        let request = RecentTradesRequest::new(Category::Linear, Some("POLUSDT"), None, None);
        let trades = market.get_recent_trades(request).await;
        if let Ok(data) = trades {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_rpi_orderbook() {
        let market: MarketData = Bybit::new(None, None);

        // Test with required parameters only (category optional)
        let request = RPIOrderbookRequest::new("BTCUSDT", None, 5);
        let rpi_orderbook = market.get_rpi_orderbook(request).await;

        match rpi_orderbook {
            Ok(data) => {
                println!("RPI Orderbook test successful!");
                println!("Symbol: {}", data.result.symbol);
                println!("Timestamp: {}", data.result.timestamp);
                println!("Update ID: {}", data.result.update_id);
                println!("Sequence: {}", data.result.sequence);
                println!(
                    "Matching Engine Timestamp: {}",
                    data.result.matching_engine_timestamp
                );

                // Check bids
                if !data.result.bids.is_empty() {
                    let best_bid = &data.result.bids[0];
                    println!(
                        "Best bid: price={}, non_rpi_size={}, rpi_size={}, total={}",
                        best_bid.price,
                        best_bid.non_rpi_size,
                        best_bid.rpi_size,
                        best_bid.total_size()
                    );
                }

                // Check asks
                if !data.result.asks.is_empty() {
                    let best_ask = &data.result.asks[0];
                    println!(
                        "Best ask: price={}, non_rpi_size={}, rpi_size={}, total={}",
                        best_ask.price,
                        best_ask.non_rpi_size,
                        best_ask.rpi_size,
                        best_ask.total_size()
                    );
                }

                // Test utility methods
                if let Some(spread) = data.result.spread() {
                    println!("Spread: {}", spread);
                }
                if let Some(mid_price) = data.result.mid_price() {
                    println!("Mid price: {}", mid_price);
                }

                println!("Total bid RPI size: {}", data.result.total_bid_rpi_size());
                println!("Total ask RPI size: {}", data.result.total_ask_rpi_size());
                println!(
                    "Total bid non-RPI size: {}",
                    data.result.total_bid_non_rpi_size()
                );
                println!(
                    "Total ask non-RPI size: {}",
                    data.result.total_ask_non_rpi_size()
                );
            }
            Err(err) => {
                println!("RPI Orderbook test failed with error: {:?}", err);
                // Don't fail the test - the API might not be available or might require authentication
            }
        }

        // Test with category specified
        let request = RPIOrderbookRequest::new("BTCUSDT", Some(Category::Spot), 3);
        let rpi_orderbook = market.get_rpi_orderbook(request).await;

        match rpi_orderbook {
            Ok(data) => {
                println!("RPI Orderbook with category test successful!");
                println!("Symbol: {}", data.result.symbol);
            }
            Err(err) => {
                println!(
                    "RPI Orderbook with category test error (might be expected): {:?}",
                    err
                );
            }
        }

        // Test convenience methods
        match RPIOrderbookRequest::spot("BTCUSDT", 10) {
            Ok(request) => {
                let rpi_orderbook = market.get_rpi_orderbook(request).await;
                if let Ok(_data) = rpi_orderbook {
                    println!("Spot RPI Orderbook test successful!");
                }
            }
            Err(err) => {
                println!("Failed to create spot request: {}", err);
            }
        }
    }

    #[test]
    async fn test_funding_rate() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let request = FundingHistoryRequest::new(Category::Linear, symbol, None, None, None);
        let funding_rate = market.get_funding_history(request).await;
        if let Ok(data) = funding_rate {
            println!("{:#?}", data.result.list.last().unwrap());
        }
    }

    #[test]
    async fn test_open_interest() {
        let market: MarketData = Bybit::new(None, None);
        let request = OpenInterestRequest::new(Category::Linear, "ETHUSDT", "1h", None, None, None);
        let open_interest = market.get_open_interest(request).await;
        if let Ok(data) = open_interest {
            println!("{:#?}", data.result.list.last().unwrap());
        }
    }

    #[test]
    async fn test_historical_volatility() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "ETH";
        let request: HistoricalVolatilityRequest<'_> =
            HistoricalVolatilityRequest::new(Some(symbol), None, None, None);
        let historical_volatility = market.get_historical_volatility(request).await;
        if let Ok(data) = historical_volatility {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_insurance() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = Some("BTC");
        let insurance = market.get_insurance(symbol).await;
        if let Ok(data) = insurance {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_risk_limit() {
        let market: MarketData =
            Bybit::new_with_config(&Config::default().set_recv_window(1000), None, None);
        let symbol = "ETHUSDT";
        let request: RiskLimitRequest<'_> = RiskLimitRequest::new(Category::Linear, Some(symbol));
        let risk_limit = market.get_risk_limit(request).await;
        if let Ok(data) = risk_limit {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_delivery_price() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let delivery_price = market
            .get_delivery_price(Category::Option, Some(symbol), None, None)
            .await;
        if let Ok(data) = delivery_price {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_longshort_ratio() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let longshort_ratio = market
            .get_longshort_ratio(Category::Linear, symbol, "4h", None)
            .await;
        if let Ok(data) = longshort_ratio {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_new_delivery_price() {
        let market: MarketData = Bybit::new(None, None);

        // Test with BTC options (defaults to USDT settlement)
        match NewDeliveryPriceRequest::btc() {
            Ok(request) => {
                let new_delivery_price = market.get_new_delivery_price(request).await;
                match new_delivery_price {
                    Ok(data) => {
                        println!("New Delivery Price test successful!");
                        println!("Category: {}", data.result.category);
                        println!("Number of records: {}", data.result.count());

                        if !data.result.is_empty() {
                            if let Some(most_recent) = data.result.most_recent() {
                                println!("Most recent delivery:");
                                println!("  Price: {}", most_recent.delivery_price);
                                println!("  Time: {}", most_recent.delivery_time);

                                if let Some(price_f64) = most_recent.delivery_price_as_f64() {
                                    println!("  Price (as f64): {}", price_f64);
                                }

                                if let Some(datetime) = most_recent.delivery_datetime() {
                                    println!("  DateTime: {}", datetime);
                                }
                            }

                            if let Some(oldest) = data.result.oldest() {
                                println!("Oldest delivery:");
                                println!("  Price: {}", oldest.delivery_price);
                                println!("  Time: {}", oldest.delivery_time);
                            }

                            // Test utility methods
                            println!("Sorted by time (ascending):");
                            for (i, item) in data.result.sorted_by_time_asc().enumerate().take(3) {
                                println!(
                                    "  {}: {} at {}",
                                    i + 1,
                                    item.delivery_price,
                                    item.delivery_time
                                );
                            }

                            println!("Sorted by time (descending):");
                            for (i, item) in data.result.sorted_by_time_desc().enumerate().take(3) {
                                println!(
                                    "  {}: {} at {}",
                                    i + 1,
                                    item.delivery_price,
                                    item.delivery_time
                                );
                            }

                            // Test find methods
                            if let Some(most_recent) = data.result.most_recent() {
                                if let Some(found) =
                                    data.result.find_by_timestamp(most_recent.delivery_time)
                                {
                                    println!("Found by timestamp: {}", found.delivery_price);
                                }

                                if let Some(closest) = data
                                    .result
                                    .find_closest_to_timestamp(most_recent.delivery_time)
                                {
                                    println!("Closest to timestamp: {}", closest.delivery_price);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("New Delivery Price test failed with error: {:?}", err);
                        // Don't fail the test - the API might not be available or might require authentication
                        // Options data might not be available on testnet
                    }
                }
            }
            Err(err) => {
                println!("Failed to create request: {}", err);
            }
        }

        // Test convenience methods
        println!("\nTesting convenience methods:");

        match NewDeliveryPriceRequest::eth() {
            Ok(request) => {
                let _ = market.get_new_delivery_price(request).await;
                println!("ETH options request created successfully");
            }
            Err(err) => {
                println!("ETH request error: {}", err);
            }
        }

        match NewDeliveryPriceRequest::sol() {
            Ok(request) => {
                let _ = market.get_new_delivery_price(request).await;
                println!("SOL options request created successfully");
            }
            Err(err) => {
                println!("SOL request error: {}", err);
            }
        }

        match NewDeliveryPriceRequest::usdt("BTC") {
            Ok(request) => {
                let _ = market.get_new_delivery_price(request).await;
                println!("BTC/USDT options request created successfully");
            }
            Err(err) => {
                println!("BTC/USDT request error: {}", err);
            }
        }

        match NewDeliveryPriceRequest::usdc("ETH") {
            Ok(request) => {
                let _ = market.get_new_delivery_price(request).await;
                println!("ETH/USDC options request created successfully");
            }
            Err(err) => {
                println!("ETH/USDC request error: {}", err);
            }
        }

        // Test builder pattern
        match NewDeliveryPriceRequest::try_new(Category::Option, "BTC", None) {
            Ok(request) => {
                let request_with_settle = request.with_settle_coin("USDC");
                let _ = market.get_new_delivery_price(request_with_settle).await;
                println!("Builder pattern test successful");
            }
            Err(err) => {
                println!("Builder pattern error: {}", err);
            }
        }
    }

    #[test]
    async fn test_adl_alert() {
        let market: MarketData = Bybit::new(None, None);

        // Test with no symbol filter (returns all symbols)
        let request = ADLAlertRequest::all_symbols();
        let adl_alert = market.get_adl_alert(request).await;

        match adl_alert {
            Ok(data) => {
                println!("ADL Alert test successful!");
                println!("Updated time: {}", data.result.updated_time);

                if let Some(datetime) = data.result.updated_datetime() {
                    println!("Updated datetime: {}", datetime);
                }

                if let Some(time_since_update) = data.result.time_since_update() {
                    println!("Time since update: {} seconds", time_since_update);

                    if let Some(is_stale) = data.result.is_stale() {
                        println!("Is stale (older than 2 minutes): {}", is_stale);
                    }
                }

                println!("Number of ADL alert items: {}", data.result.count());

                if !data.result.is_empty() {
                    // Display first few items
                    for (i, item) in data.result.list.iter().take(3).enumerate() {
                        println!("Item {}:", i + 1);
                        println!("  Symbol: {}", item.symbol);
                        println!("  Coin: {}", item.coin);
                        println!("  Balance: {}", item.balance);
                        println!("  PnL Ratio: {}", item.pnl_ratio);
                        println!("  Insurance PnL Ratio: {}", item.insurance_pnl_ratio);
                        println!("  ADL Trigger Threshold: {}", item.adl_trigger_threshold);
                        println!("  ADL Stop Ratio: {}", item.adl_stop_ratio);
                        println!("  Max Balance (deprecated): {}", item.max_balance);

                        // Test utility methods
                        if let Some(balance) = item.balance_as_f64() {
                            println!("  Balance (as f64): {}", balance);
                        }

                        let (
                            contract_triggered,
                            contract_stopped,
                            equity_triggered,
                            equity_stopped,
                        ) = item.adl_status();
                        println!("  Contract PnL Drawdown ADL:");
                        println!("    Triggered: {:?}", contract_triggered);
                        println!("    Stopped: {:?}", contract_stopped);
                        println!("  Insurance Pool Equity ADL:");
                        println!("    Triggered: {:?}", equity_triggered);
                        println!("    Stopped: {:?}", equity_stopped);

                        if let Some(any_triggered) = item.is_any_adl_triggered() {
                            println!("  Any ADL triggered: {}", any_triggered);
                        }

                        if let Some(all_stopped) = item.is_all_adl_stopped() {
                            println!("  All ADL stopped: {}", all_stopped);
                        }
                    }

                    // Test summary utility methods
                    let triggered_items = data.result.triggered_items();
                    println!(
                        "Number of items with ADL triggered: {}",
                        triggered_items.len()
                    );

                    let stopped_items = data.result.stopped_items();
                    println!(
                        "Number of items with all ADL stopped: {}",
                        stopped_items.len()
                    );

                    // Test filtering by coin
                    let usdt_items = data.result.filter_by_coin("USDT");
                    println!("Number of USDT items: {}", usdt_items.len());

                    // Test find by symbol
                    if let Some(first_item) = data.result.list.first() {
                        if let Some(found_item) = data.result.find_by_symbol(&first_item.symbol) {
                            println!(
                                "Found item for symbol {}: balance = {}",
                                found_item.symbol, found_item.balance
                            );
                        }
                    }
                }
            }
            Err(err) => {
                println!("ADL Alert test failed with error: {:?}", err);
                // Don't fail the test - the API might not be available or might require authentication
            }
        }

        // Test with symbol filter
        let request = ADLAlertRequest::btcusdt();
        let adl_alert = market.get_adl_alert(request).await;

        match adl_alert {
            Ok(data) => {
                println!("ADL Alert with symbol filter test successful!");
                println!("Number of items: {}", data.result.count());

                if !data.result.is_empty() {
                    let item = &data.result.list[0];
                    println!(
                        "Filtered item - Symbol: {}, Balance: {}",
                        item.symbol, item.balance
                    );
                }
            }
            Err(err) => {
                println!(
                    "ADL Alert with symbol filter test error (might be expected): {:?}",
                    err
                );
            }
        }

        // Test convenience methods
        println!("\nTesting convenience methods:");

        for symbol_request in [
            ADLAlertRequest::ethusdt(),
            ADLAlertRequest::solusdt(),
            ADLAlertRequest::xrpusdt(),
            ADLAlertRequest::adausdt(),
        ] {
            let _ = market.get_adl_alert(symbol_request).await;
            // Don't check result - just testing that requests can be created
        }

        // Test builder pattern
        let request = ADLAlertRequest::default()
            .with_symbol("BTCUSDT")
            .without_symbol();
        let _ = market.get_adl_alert(request).await;
        println!("Builder pattern test completed");
    }
}
