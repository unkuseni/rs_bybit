//! # REST API Usage Examples
//!
//! This example demonstrates how to use the `rs_bybit` crate to interact
//! with the Bybit V5 REST API.
//!
//! ## Setup
//!
//! Bybit API credentials are required for authenticated endpoints:
//!
//! ```bash
//! export BYBIT_API_KEY="your_api_key"
//! export BYBIT_API_SECRET="your_api_secret"
//! ```
//!
//! ## Running
//!
//! ```bash
//! # Public endpoints only (no API keys needed)
//! cargo run --example rest_api
//!
//! # With API keys for authenticated endpoints
//! BYBIT_API_KEY="your_key" BYBIT_API_SECRET="your_secret" cargo run --example rest_api
//! ```

use bybit::prelude::*;
use std::env;

/// Retrieves API credentials from environment variables.
fn get_credentials() -> (Option<String>, Option<String>) {
    let api_key = env::var("BYBIT_API_KEY").ok();
    let api_secret = env::var("BYBIT_API_SECRET").ok();
    (api_key, api_secret)
}

/// Demonstrates general/market endpoints (no authentication required).
async fn run_general_examples() -> Result<(), BybitError> {
    println!("\n═══════════════════════════════════════");
    println!("  GENERAL & MARKET DATA ENDPOINTS");
    println!("═══════════════════════════════════════\n");

    let general: General = Bybit::new(None, None);
    let market: MarketData = Bybit::new(None, None);

    // --- Server Time ---
    println!("▶ Server Time:");
    match general.get_server_time().await {
        Ok(response) => println!("  Server time: {} seconds", response.result.time_second),
        Err(e) => println!("  Error: {e}"),
    }

    // --- Server Datetime ---
    println!("\n▶ Server Datetime:");
    match general.get_server_datetime().await {
        Ok(dt) => println!("  Server datetime: {dt}"),
        Err(e) => println!("  Error: {e}"),
    }

    // --- Server Time in Milliseconds ---
    println!("\n▶ Server Time (millis):");
    match general.get_server_time_millis().await {
        Ok(ms) => println!("  Server time: {ms} ms"),
        Err(e) => println!("  Error: {e}"),
    }

    // --- API Timestamp ---
    println!("\n▶ API Timestamp (for signing):");
    match general.get_api_timestamp().await {
        Ok(ts) => println!("  Timestamp: {ts}"),
        Err(e) => println!("  Error: {e}"),
    }

    // --- Ping ---
    println!("\n▶ Ping:");
    match general.ping().await {
        Ok(true) => println!("  ✓ Connected to Bybit API"),
        Ok(false) => println!("  ✗ Bybit API responded with an error"),
        Err(e) => println!("  ✗ Connection failed: {e}"),
    }

    // --- System Status ---
    println!("\n▶ System Status:");
    match general.get_system_status(None, None).await {
        Ok(response) => {
            for entry in &response.result.list {
                println!(
                    "  ID: {}, Title: {}, State: {}",
                    entry.id, entry.title, entry.state
                );
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Kline / Candlestick Data ---
    println!("\n▶ Kline Data (BTCUSDT, 1h, 2024-01-01 to 2024-01-05):");
    match market
        .get_klines(KlineRequest::new(
            Some(Category::Linear),
            "BTCUSDT",
            Interval::H1,
            Some("010124"),
            Some("050124"),
            Some(5),
        ))
        .await
    {
        Ok(response) => {
            for kline in &response.result.list {
                println!(
                    "  O:{open:>10.2} H:{high:>10.2} L:{low:>10.2} C:{close:>10.2} V:{volume:>10.4}",
                    open = kline.open_price,
                    high = kline.high_price,
                    low = kline.low_price,
                    close = kline.close_price,
                    volume = kline.volume,
                );
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Instrument Info ---
    println!("\n▶ Instrument Info (Spot, ETHUSDT):");
    match market
        .get_instrument_info(InstrumentRequest::new(
            Category::Spot,
            Some("ETHUSDT"),
            None,
            None,
            None,
            None,
            None,
        ))
        .await
    {
        Ok(response) => match response.result {
            InstrumentInfo::Spot(spot) => {
                if let Some(instrument) = spot.list.first() {
                    println!("  Symbol: {}", instrument.symbol);
                    println!("  Status: {}", instrument.status);
                    println!("  Base Coin: {}", instrument.base_coin);
                    println!("  Quote Coin: {}", instrument.quote_coin);
                }
            }
            InstrumentInfo::Futures(futures) => {
                if let Some(instrument) = futures.list.first() {
                    println!("  Symbol: {}", instrument.symbol);
                    println!("  Status: {}", instrument.status);
                }
            }
            _ => {}
        },
        Err(e) => println!("  Error: {e}"),
    }

    // --- Order Book ---
    println!("\n▶ Order Book (BTCUSDT, depth 5):");
    match market
        .get_depth(OrderbookRequest::new("BTCUSDT", Category::Linear, Some(5)))
        .await
    {
        Ok(response) => {
            let ob = &response.result;
            println!("  Timestamp: {}", ob.timestamp);
            println!(
                "  Best bid: {:?}",
                ob.bids.first().map(|b| (b.price, b.qty))
            );
            println!(
                "  Best ask: {:?}",
                ob.asks.first().map(|a| (a.price, a.qty))
            );
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Ticker ---
    println!("\n▶ Ticker (Linear, BTCUSDT):");
    match market
        .get_tickers(TickerRequest::new(
            Category::Linear,
            Some("BTCUSDT"),
            None,
            None,
        ))
        .await
    {
        Ok(response) => {
            if let Some(ticker) = response.result.list.first() {
                match ticker {
                    TickerData::Futures(t) => {
                        println!(
                            "  Last: {:.2}, Bid: {:.2}, Ask: {:.2}, Vol24h: {:.2}",
                            t.last_price, t.bid_price, t.ask_price, t.volume_24h,
                        );
                        println!("  Funding rate: {}", t.funding_rate);
                    }
                    _ => println!("  Unexpected ticker type"),
                }
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Open Interest ---
    println!("\n▶ Open Interest (Linear, BTCUSDT, 1h):");
    match market
        .get_open_interest(OpenInterestRequest::new(
            Category::Linear,
            "BTCUSDT",
            "1h",
            None,
            None,
            Some(3),
        ))
        .await
    {
        Ok(response) => {
            for oi in &response.result.list {
                println!("  Time: {}, OI: {}", oi.timestamp, oi.open_interest);
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Funding History ---
    println!("\n▶ Funding History (Linear, BTCUSDT):");
    match market
        .get_funding_history(FundingHistoryRequest::new(
            Category::Linear,
            "BTCUSDT",
            None,
            None,
            Some(3),
        ))
        .await
    {
        Ok(response) => {
            for fr in &response.result.list {
                println!(
                    "  Time: {}, Rate: {}",
                    fr.funding_rate_timestamp, fr.funding_rate
                );
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Recent Trades ---
    println!("\n▶ Recent Trades (BTCUSDT, 5):");
    match market
        .get_recent_trades(RecentTradesRequest::new(
            Category::Linear,
            Some("BTCUSDT"),
            None,
            Some(5),
        ))
        .await
    {
        Ok(response) => {
            for trade in &response.result.list {
                println!(
                    "  {:?} | price: {:>10.2} | size: {:>8.4} | time: {}",
                    trade.side, trade.price, trade.size, trade.time,
                );
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Insurance ---
    println!("\n▶ Insurance:");
    match market.get_insurance(None).await {
        Ok(response) => {
            for pool in &response.result.list {
                println!(
                    "  Coin: {}, Balance: {:.4}, Value: {}",
                    pool.coin, pool.balance, pool.value
                );
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Risk Limit ---
    println!("\n▶ Risk Limit (Linear):");
    match market
        .get_risk_limit(RiskLimitRequest::new(Category::Linear, None))
        .await
    {
        Ok(response) => {
            for rl in &response.result.list {
                println!(
                    "  Symbol: {}, ID: {}, MaxLev: {:.1}, IM: {:.4}, MM: {:.4}",
                    rl.symbol, rl.id, rl.max_leverage, rl.initial_margin, rl.maintenance_margin,
                );
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    Ok(())
}

/// Demonstrates authenticated endpoints.
async fn run_authenticated_examples(api_key: &str, api_secret: &str) -> Result<(), BybitError> {
    println!("\n═══════════════════════════════════════");
    println!("  AUTHENTICATED ENDPOINTS");
    println!("═══════════════════════════════════════\n");

    let trader: Trader = Bybit::new(Some(api_key.into()), Some(api_secret.into()));
    let position: PositionManager = Bybit::new(Some(api_key.into()), Some(api_secret.into()));
    let account: AccountManager = Bybit::new(Some(api_key.into()), Some(api_secret.into()));

    // --- Wallet Balance ---
    println!("▶ Wallet Balance:");
    match account.get_wallet_balance("UNIFIED", None).await {
        Ok(response) => {
            for wallet in &response.result.list {
                println!("  Total equity: {}", wallet.total_equity);
                for coin in &wallet.coin {
                    println!("    {}: wallet={}", coin.coin, coin.wallet_balance);
                }
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Open Orders ---
    println!("\n▶ Open Orders (Linear):");
    match trader
        .get_open_orders(OpenOrdersRequest::custom(
            Category::Linear,
            "BTCUSDT",
            None,
            None,
            None,
            None,
            0,
            None,
            None,
        ))
        .await
    {
        Ok(response) => {
            if response.result.list.is_empty() {
                println!("  No open orders");
            } else {
                for order in &response.result.list {
                    println!(
                        "  {} | {:?} | qty: {} | status: {}",
                        order.symbol, order.side, order.qty, order.order_status,
                    );
                }
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Position Info ---
    println!("\n▶ Position Info (Linear, BTCUSDT):");
    match position
        .get_info(PositionRequest::new(
            Category::Linear,
            Some("BTCUSDT"),
            None,
            None,
            None,
        ))
        .await
    {
        Ok(response) => {
            if response.result.list.is_empty() {
                println!("  No open positions");
            } else {
                for pos in &response.result.list {
                    println!(
                        "  {} | {:?} | size: {} | entry: {:.2} | liq: {:.2} | pnl: {}",
                        pos.symbol,
                        pos.side,
                        pos.size,
                        pos.avg_price.unwrap(),
                        pos.liq_price.unwrap(),
                        pos.unrealised_pnl.unwrap(),
                    );
                }
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Fee Rate ---
    println!("\n▶ Fee Rate (Spot, BTCUSDT):");
    match account
        .get_fee_rate(Category::Spot, Some("BTCUSDT".to_string()))
        .await
    {
        Ok(response) => {
            for fee in &response.result.list {
                println!(
                    "  Symbol: {}, Maker: {}, Taker: {}",
                    fee.symbol, fee.maker_fee_rate, fee.taker_fee_rate,
                );
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Order History ---
    println!("\n▶ Order History (Linear, last 5):");
    match trader
        .get_order_history(OrderHistoryRequest::new(
            Category::Linear,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(5),
            None,
            None,
        ))
        .await
    {
        Ok(response) => {
            if response.result.list.is_empty() {
                println!("  No order history");
            } else {
                for order in &response.result.list {
                    println!(
                        "  {} | {:?} | qty: {} | status: {} | created: {}",
                        order.symbol, order.side, order.qty, order.order_status, order.created_time,
                    );
                }
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    // --- Trade History ---
    println!("\n▶ Trade History (Linear, last 5):");
    match trader
        .get_trade_history(TradeHistoryRequest::new(
            Category::Linear,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(5),
        ))
        .await
    {
        Ok(response) => {
            if response.result.list.is_empty() {
                println!("  No trade history");
            } else {
                for trade in &response.result.list {
                    println!(
                        "  {} | {:?} | exec_qty: {} | price: {} | time: {}",
                        trade.symbol, trade.side, trade.exec_qty, trade.exec_price, trade.exec_time,
                    );
                }
            }
        }
        Err(e) => println!("  Error: {e}"),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), BybitError> {
    println!("═══════════════════════════════════════");
    println!(
        "  rs_bybit v{} — REST API Examples",
        env!("CARGO_PKG_VERSION")
    );
    println!("═══════════════════════════════════════");

    // Public endpoints (no auth needed)
    run_general_examples().await?;

    // Authenticated endpoints (API keys required)
    let (api_key, api_secret) = get_credentials();
    match (api_key, api_secret) {
        (Some(key), Some(secret)) if !key.is_empty() && !secret.is_empty() => {
            run_authenticated_examples(&key, &secret).await?;
        }
        _ => {
            println!(
                "\n⚠  Skipping authenticated examples — set BYBIT_API_KEY and BYBIT_API_SECRET"
            );
        }
    }

    println!("\n═══════════════════════════════════════");
    println!("  All examples completed.");
    println!("═══════════════════════════════════════\n");

    Ok(())
}
