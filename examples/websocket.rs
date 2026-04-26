//! # WebSocket API Examples for Bybit V5
//!
//! This example demonstrates how to use the WebSocket API to subscribe to
//! real-time data streams.
//!
//! ## Running
//!
//! ```bash
//! # Public trades (no API keys needed)
//! cargo run --example websocket trades
//!
//! # Order book stream
//! cargo run --example websocket orderbook
//!
//! # Ticker stream
//! cargo run --example websocket ticker
//!
//! # Kline stream
//! cargo run --example websocket kline
//!
//! # Private streams (API keys required)
//! BYBIT_API_KEY="your_key" BYBIT_API_SECRET="your_secret" \
//!   cargo run --example websocket private
//!
//! # Ping public endpoint
//! cargo run --example websocket ping
//! ```

use bybit::prelude::*;
use std::env;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

/// Available demo modes.
enum Mode {
    Trades,
    OrderBook,
    Ticker,
    Kline,
    Private,
    Ping,
}

impl Mode {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "trades" | "trade" => Some(Self::Trades),
            "orderbook" | "order-book" | "depth" => Some(Self::OrderBook),
            "ticker" => Some(Self::Ticker),
            "kline" | "klines" | "candles" => Some(Self::Kline),
            "private" | "auth" => Some(Self::Private),
            "ping" => Some(Self::Ping),
            _ => None,
        }
    }
}

/// Run a WebSocket demo for a given duration, then abort.
async fn run_with_timeout(duration_secs: u64, fut: impl std::future::Future<Output = ()>) {
    if duration_secs > 0 {
        tokio::select! {
            _ = fut => {}
            _ = tokio::time::sleep(Duration::from_secs(duration_secs)) => {
                println!("⏱  Time limit reached ({duration_secs}s).");
            }
        }
    } else {
        fut.await;
    }
}

// ---------------------------------------------------------------------------
// Demo 1 – Public trades
// ---------------------------------------------------------------------------
async fn demo_trades(symbol: &str, duration: u64) {
    println!("▶ Subscribing to public trades for {symbol} …");
    let ws: Stream = Bybit::new(None, None);
    let symbol_owned = symbol.to_owned();

    run_with_timeout(duration, async move {
        let topic = format!("publicTrade.{symbol_owned}");
        let sub = Subscription {
            op: "subscribe",
            args: vec![&topic],
        };
        let _ = ws
            .ws_subscribe(sub, Category::Linear, |event| {
                if let WebsocketEvents::TradeEvent(trade) = event {
                    for t in &trade.data {
                        println!(
                            "  Trade | {:>4} | {:>8.4} @ {:>10.2}",
                            t.side, t.volume, t.price,
                        );
                    }
                }
                Ok(())
            })
            .await;
    })
    .await;
}

// ---------------------------------------------------------------------------
// Demo 2 – Order book delta stream via ws_orderbook
// ---------------------------------------------------------------------------
async fn demo_orderbook(symbol: &str, duration: u64) {
    println!("▶ Subscribing to order book (level 50) for {symbol} …");
    let ws: Stream = Bybit::new(None, None);
    let (tx, mut rx) = mpsc::unbounded_channel();
    let symbol_owned = symbol.to_owned();

    let ws_handle = tokio::spawn(async move {
        ws.ws_orderbook(vec![(50, symbol_owned.as_str())], Category::Linear, tx)
            .await
            .ok();
    });

    let deadline = tokio::time::Instant::now() + Duration::from_secs(duration);
    while tokio::time::Instant::now() < deadline {
        match timeout(Duration::from_secs(5), rx.recv()).await {
            Ok(Some(update)) => {
                let best_ask = update.data.asks.first().map(|a| a.price).unwrap_or(0.0);
                let best_bid = update.data.bids.first().map(|b| b.price).unwrap_or(0.0);
                println!(
                    "  OrderBook | type: {} | top ask: {:.2} | top bid: {:.2}",
                    update.event_type, best_ask, best_bid,
                );
            }
            Ok(None) => break,
            Err(_) => println!("  (no order book update in 5s)"),
        }
    }

    let _ = ws_handle.await;
}

// ---------------------------------------------------------------------------
// Demo 3 – Ticker stream
// ---------------------------------------------------------------------------
async fn demo_ticker(symbol: &str, duration: u64) {
    println!("▶ Subscribing to ticker for {symbol} …");
    let ws: Stream = Bybit::new(None, None);
    let symbol_owned = symbol.to_owned();

    run_with_timeout(duration, async move {
        let topic = format!("tickers.{symbol_owned}");
        let sub = Subscription {
            op: "subscribe",
            args: vec![&topic],
        };
        let _ = ws
            .ws_subscribe(sub, Category::Linear, |event| {
                if let WebsocketEvents::TickerEvent(ticker) = event {
                    match ticker.data {
                        // `Ticker.data` is of type `Ticker` (not `TickerData`)
                        Ticker::Futures(ref t) => {
                            println!(
                                "  Ticker(Futures) | last: {:>10.2} | bid: {:>10.2} | ask: {:>10.2} | vol24: {:>12.2} | funding: {}",
                                t.last_price, t.bid_price, t.ask_price, t.volume_24h, t.funding_rate,
                            );
                        }
                        Ticker::Spot(ref t) => {
                            println!(
                                "  Ticker(Spot)    | last: {:>10.2} | high: {:>10.2} | low: {:>10.2}",
                                t.last_price, t.high_price_24h, t.low_price_24h,
                            );
                        }
                        Ticker::Options(ref t) => {
                            println!(
                                "  Ticker(Options) | symbol: {} | bid: {:>10} | ask: {:>10}",
                                t.symbol, t.bid1_price, t.ask1_price,
                            );
                        }
                        Ticker::Linear(_) => {
                            // Linear ticker is an untagged enum: Snapshot or Delta
                            // Just print the symbol for simplicity
                            println!("  Ticker(Linear) | symbol: (snapshot/delta)");
                        }
                    }
                }
                Ok(())
            })
            .await;
    })
    .await;
}

// ---------------------------------------------------------------------------
// Demo 4 – Kline / candlestick stream
// ---------------------------------------------------------------------------
async fn demo_kline(symbol: &str, duration: u64) {
    println!("▶ Subscribing to 1m klines for {symbol} …");
    let ws: Stream = Bybit::new(None, None);
    let symbol_owned = symbol.to_owned();

    run_with_timeout(duration, async move {
        let topic = format!("kline.1.{symbol_owned}");
        let sub = Subscription {
            op: "subscribe",
            args: vec![&topic],
        };
        let _ = ws
            .ws_subscribe(sub, Category::Linear, |event| {
                if let WebsocketEvents::KlineEvent(kline) = event {
                    for k in &kline.data {
                        println!(
                            "  Kline | O: {:>10.2} | H: {:>10.2} | L: {:>10.2} | C: {:>10.2} | Vol: {:>10.2} | start: {}",
                            k.open, k.high, k.low, k.close, k.volume, k.start,
                        );
                    }
                }
                Ok(())
            })
            .await;
    })
    .await;
}

// ---------------------------------------------------------------------------
// Demo 5 – Private streams (orders, positions, executions, wallet)
// ---------------------------------------------------------------------------
async fn demo_private(api_key: &str, secret: &str, duration: u64) {
    println!("▶ Connecting to private WebSocket streams …");
    let ws: Stream = Bybit::new(Some(api_key.into()), Some(secret.into()));
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        // Start the private connection with command control
        tokio::spawn(async move {
            ws.ws_priv_subscribe_with_commands(cmd_rx, move |event| {
                let tx = tx.clone();
                let _ = tx.send(event);
                Ok(())
            })
            .await
            .ok();
        });

        // Wait for connection to establish, then subscribe to topics
        tokio::time::sleep(Duration::from_secs(2)).await;
        for topic in &["order", "position", "execution", "wallet"] {
            println!("  → Subscribing to private topic: {topic}");
            let _ = cmd_tx.send(RequestType::Subscribe(Subscription {
                op: "subscribe",
                args: vec![topic],
            }));
        }
    });

    // Read events for the specified duration
    let deadline = tokio::time::Instant::now() + Duration::from_secs(duration);
    while tokio::time::Instant::now() < deadline {
        match timeout(Duration::from_secs(10), rx.recv()).await {
            Ok(Some(event)) => match event {
                WebsocketEvents::OrderEvent(order) => {
                    // `order.data` is `Vec<OrderData>`
                    for d in &order.data {
                        println!(
                            "  Order    | {} | {:?} | qty: {} | status: {}",
                            d.symbol, d.side, d.qty, d.order_status,
                        );
                    }
                }
                WebsocketEvents::PositionEvent(pos) => {
                    // `pos.data` is `Vec<PositionData>`
                    for d in &pos.data {
                        println!(
                            "  Position | {} | {:?} | size: {} | pnl: {}",
                            d.symbol, d.side, d.size, d.unrealised_pnl,
                        );
                    }
                }
                WebsocketEvents::ExecutionEvent(exec) => {
                    // `exec.data` is `Vec<ExecutionData>`
                    for d in &exec.data {
                        println!(
                            "  Exec     | {} | {} | qty: {} | price: {}",
                            d.symbol, d.side, d.exec_qty, d.exec_price,
                        );
                    }
                }
                WebsocketEvents::Wallet(wallet) => {
                    // `wallet.data` is `Vec<WalletData>`, each has `coin: Vec<CoinData>`
                    for d in &wallet.data {
                        for c in &d.coin {
                            println!(
                                "  Wallet   | coin: {} | wallet: {} | equity: {}",
                                c.coin, c.wallet_balance, c.equity,
                            );
                        }
                    }
                }
                _ => println!("  Other private event received"),
            },
            Ok(None) => break,
            Err(_) => println!("  (no private event in 10s)"),
        }
    }
}

// ---------------------------------------------------------------------------
// Demo 6 – Ping
// ---------------------------------------------------------------------------
async fn demo_ping() {
    let ws: Stream = Bybit::new(None, None);
    match ws.ws_ping(false).await {
        Ok(_) => println!("  ✓ Ping successful"),
        Err(e) => eprintln!("  ✗ Ping failed: {e:?}"),
    }
}

// ---------------------------------------------------------------------------
// Help
// ---------------------------------------------------------------------------
fn print_usage() {
    eprintln!(
        r#"Usage: cargo run --example websocket <MODE> [SYMBOL] [DURATION]

Modes:
  trades     – Subscribe to public trades
  orderbook  – Subscribe to order book delta stream
  ticker     – Subscribe to ticker updates
  kline      – Subscribe to 1m candlesticks
  private    – Subscribe to private streams (needs BYBIT_API_KEY / BYBIT_API_SECRET)
  ping       – Ping the public WebSocket endpoint

Symbol (default: BTCUSDT)
Duration in seconds (default: 30, 0 = run forever)

Environment:
  BYBIT_API_KEY       API key (required for private mode)
  BYBIT_API_SECRET    API secret (required for private mode)
"#,
    );
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let mode = match Mode::from_str(&args[1]) {
        Some(m) => m,
        None => {
            eprintln!("Unknown mode: {}", args[1]);
            print_usage();
            std::process::exit(1);
        }
    };

    let symbol = args.get(2).map(|s| s.as_str()).unwrap_or("BTCUSDT");
    let duration: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(30);

    match mode {
        Mode::Trades => demo_trades(symbol, duration).await,
        Mode::OrderBook => demo_orderbook(symbol, duration).await,
        Mode::Ticker => demo_ticker(symbol, duration).await,
        Mode::Kline => demo_kline(symbol, duration).await,
        Mode::Private => {
            let api_key = env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY required");
            let secret = env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET required");
            demo_private(&api_key, &secret, duration).await;
        }
        Mode::Ping => demo_ping().await,
    }
}
