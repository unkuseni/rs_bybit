# rs_bybit — Bybit V5 API bindings for Rust

[![Crates.io][crates-badge]][crates-url]
[![License: MIT][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/rs_bybit.svg
[crates-url]: https://crates.io/crates/rs_bybit
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/unkuseni/rs_bybit/blob/main/LICENSE

Rust client library for the [Bybit V5 API](https://bybit-exchange.github.io/docs/v5/intro). Supports both REST and WebSocket interfaces for trading perpetual futures, spot, and options on Bybit.

> [!CAUTION]
> This is a personal project, use at your own risk. Neither the original author nor any contributors shall be held responsible for your investment losses. Cryptocurrency trading is subject to high market risk.

---

## Table of Contents

- [Features](#features)
- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [REST API (Public)](#rest-api-public)
  - [REST API (Authenticated)](#rest-api-authenticated)
  - [WebSocket (Public Streams)](#websocket-public-streams)
  - [WebSocket (Private Streams)](#websocket-private-streams)
- [Architecture](#architecture)
- [Examples](#examples)
- [Running Tests](#running-tests)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

---

## Features

### REST API

- **Market Data** — Klines, tickers, order book, instruments info, funding rates, open interest, recent trades, insurance, risk limits, and more
- **Trade** — Place, amend, cancel orders (single & batch), order history, trade history, pre-check orders, borrow quota, DCP configuration
- **Position** — Get position info, set leverage, margin mode, risk limit, trading stop, add/reduce margin, closed PnL, confirm pending MMR, move positions
- **Account** — Wallet balance, fee rates, borrow history, transaction logs, collateral info, coin Greeks, upgrade to UTA, set margin/spot hedging modes
- **Asset** — Coin balance, settlement/delivery records, coin info, exchange records, internal & universal transfers, deposit/withdrawal management, sub-UID queries
- **General** — Server time, ping, system status

### WebSocket API

- **Public Streams** — Order book (snapshot + delta), public trades, tickers (linear, spot, options, futures), klines, liquidations, all-liquidations, insurance pool updates, price limits, ADL alerts, system status, RPI order book
- **Private Streams** — Orders, positions, executions, wallet, trade stream, fast executions
- **Dynamic Subscription** — Subscribe/unsubscribe to topics without reconnecting
- **Order Management** — Create, amend, cancel orders over WebSocket (including batch operations)

---

## Quick Start

```rust
use bybit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), BybitError> {
    // Public endpoints — no API keys needed
    let market: MarketData = Bybit::new(None, None);

    let time = market.get_server_time().await?;
    println!("Server time: {}s", time.result.time_second);

    // Authenticated endpoints — requires API keys
    let trader: Trader = Bybit::new(
        Some("YOUR_API_KEY".into()),
        Some("YOUR_SECRET".into()),
    );

    let wallet = trader.get_wallet_balance("UNIFIED", None).await?;
    println!("Wallet: {:?}", wallet);

    Ok(())
}
```

---

## Prerequisites

- **Rust** 1.70+ (edition 2021)
- **OpenSSL** (for native TLS support in WebSocket connections)
- A Bybit account with API keys for authenticated endpoints

---

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rs_bybit = "0.4"
tokio = { version = "1", features = ["full"] }
```

> **Note**: You also need `tokio` in your project since all API calls are async.

---

## Configuration

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `BYBIT_API_KEY` | No* | Bybit API key for authenticated endpoints |
| `BYBIT_API_SECRET` | No* | Bybit API secret for authenticated endpoints |

\*Required only when using authenticated endpoints (trading, positions, account, asset, private WebSocket streams).

### Custom Configuration

Use `Bybit::new_with_config()` to customize endpoints and recv_window:

```rust
use bybit::prelude::*;

let config = Config::testnet(); // Testnet endpoints
// or
let config = Config::default()
    .set_recv_window(10000);    // Custom recv window (ms)

let market: MarketData = Bybit::new_with_config(&config, None, None);
```

The `Config` struct supports:
- `Config::default()` — Production endpoints (`https://api.bybit.com`, `wss://stream.bybit.com/v5`)
- `Config::testnet()` — Testnet endpoints (`https://api-testnet.bybit.com`, `wss://stream-testnet.bybit.com/v5`)
- `Config::new(rest_api, ws, recv_window)` — Fully custom

---

## Usage

### REST API (Public)

No authentication required for market data endpoints:

```rust
use bybit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), BybitError> {
    let market: MarketData = Bybit::new(None, None);

    // Server time
    let time = market.get_server_time().await?;
    println!("Server time: {}s", time.result.time_second);

    // Ping
    let connected = market.ping().await?;
    println!("Connected: {connected}");

    // System status
    let status = market.get_system_status(None, None).await?;
    for entry in &status.result.list {
        println!("  {} — {} [{:?}]", entry.id, entry.title, entry.state);
    }

    // Klines (candlesticks)
    let klines = market
        .get_klines(KlineRequest::new(
            Some(Category::Linear),
            "BTCUSDT",
            Interval::H1,
            Some("010124"),
            Some("050124"),
            Some(10),
        ))
        .await?;
    for k in &klines.result.list {
        println!("O:{:.2} H:{:.2} L:{:.2} C:{:.2}", k.open_price, k.high_price, k.low_price, k.close_price);
    }

    // Order book
    let ob = market
        .get_depth(OrderbookRequest::new("BTCUSDT", Category::Linear, Some(5)))
        .await?;
    if let Some(best_bid) = ob.result.bids.first() {
        println!("Best bid: {:.2} @ {:.4}", best_bid.price, best_bid.qty);
    }

    // Ticker
    let tickers = market
        .get_tickers(TickerRequest::new(Category::Linear, Some("BTCUSDT"), None, None))
        .await?;
    if let Some(TickerData::Futures(t)) = tickers.result.list.first() {
        println!("Last: {:.2}, Funding: {}", t.last_price, t.funding_rate);
    }

    // Recent trades
    let trades = market
        .get_recent_trades(RecentTradesRequest::new(Category::Linear, Some("BTCUSDT"), None, Some(5)))
        .await?;
    for t in &trades.result.list {
        println!("{:?} {:.4} @ {:.2}", t.side, t.size, t.price);
    }

    Ok(())
}
```

### REST API (Authenticated)

Requires API keys for trading, account, and position endpoints:

```rust
use bybit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), BybitError> {
    let api_key = std::env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY required");
    let secret = std::env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET required");

    let trader: Trader = Bybit::new(Some(api_key), Some(secret));

    // Wallet balance
    let wallet = trader.get_wallet_balance("UNIFIED", None).await?;
    for w in &wallet.result.list {
        println!("Equity: {}", w.total_equity);
    }

    // Open orders
    let orders = trader
        .get_open_orders(OpenOrdersRequest::custom(
            Category::Linear, "BTCUSDT",
            None, None, None, None, 0, None, None,
        ))
        .await?;
    println!("Open orders: {}", orders.result.list.len());

    // Order history (last 5)
    let history = trader
        .get_order_history(OrderHistoryRequest::new(
            Category::Linear, None, None, None, None, None, None, None, None, None, Some(5),
        ))
        .await?;
    for o in &history.result.list {
        println!("{:?} {} qty:{} status:{}", o.side, o.symbol, o.qty, o.order_status);
    }

    // Place a limit order
    let order = trader
        .place_futures_limit_order(Category::Linear, "ETHUSDT", Side::Buy, 0.1, 2500.0, 0)
        .await?;
    println!("Order placed: {}", order.result.order_id);

    Ok(())
}
```

### WebSocket (Public Streams)

Subscribe to real-time market data:

```rust
use bybit::prelude::*;

#[tokio::main]
async fn main() {
    let ws: Stream = Bybit::new(None, None);

    let sub = Subscription {
        op: "subscribe",
        args: vec!["publicTrade.BTCUSDT"],
    };

    ws.ws_subscribe(sub, Category::Linear, |event| {
        if let WebsocketEvents::TradeEvent(trade) = event {
            for t in &trade.data {
                println!("Trade {:?} {:.4} @ {:.2}", t.side, t.volume, t.price);
            }
        }
        Ok(())
    })
    .await
    .ok();
}
```

For order book streams, use the dedicated helper:

```rust
use bybit::prelude::*;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let ws: Stream = Bybit::new(None, None);
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        ws.ws_orderbook(vec![(50, "BTCUSDT")], Category::Linear, tx)
            .await
            .ok();
    });

    while let Some(update) = rx.recv().await {
        let asks = &update.data.asks;
        let bids = &update.data.bids;
        if let (Some(ask), Some(bid)) = (asks.first(), bids.first()) {
            println!("Ask: {:.2} Bid: {:.2}", ask.price, bid.price);
        }
    }
}
```

### WebSocket (Private Streams)

Requires API keys. Subscribe to orders, positions, executions, and wallet updates with dynamic subscription control:

```rust
use bybit::prelude::*;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("BYBIT_API_KEY").unwrap();
    let secret = std::env::var("BYBIT_API_SECRET").unwrap();
    let ws: Stream = Bybit::new(Some(api_key), Some(secret));

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();

    // Start private connection with command control
    tokio::spawn(async move {
        ws.ws_priv_subscribe_with_commands(cmd_rx, move |event| {
            let _ = event_tx.send(event);
            Ok(())
        })
        .await
        .ok();
    });

    // Subscribe to private topics
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    for topic in &["order", "position", "execution", "wallet"] {
        let _ = cmd_tx.send(RequestType::Subscribe(Subscription {
            op: "subscribe",
            args: vec![topic],
        }));
    }

    // Process events
    while let Some(event) = event_rx.recv().await {
        match event {
            WebsocketEvents::OrderEvent(o) => {
                for d in &o.data { println!("Order: {} {:?}", d.symbol, d.side); }
            }
            WebsocketEvents::PositionEvent(p) => {
                for d in &p.data { println!("Position: {} size:{}", d.symbol, d.size); }
            }
            WebsocketEvents::Wallet(w) => {
                for d in &w.data { println!("Wallet: {:?}", d.coin); }
            }
            _ => {}
        }
    }
}
```

---

## Architecture

The library is organized around manager structs, each providing a focused set of API methods:

```
rs_bybit
├── src/
│   ├── lib.rs          # Re-exports: pub mod prelude
│   ├── api.rs          # API enums (Market, Trade, Position, etc.) + Bybit trait
│   ├── client.rs       # HTTP/WebSocket client (signing, requests)
│   ├── config.rs       # Config with rest/ws endpoints & recv_window
│   ├── errors.rs       # BybitError + BybitContentError
│   ├── general.rs      # General — server time, ping, system status
│   ├── market.rs       # MarketData — klines, order book, tickers, etc.
│   ├── trade.rs        # Trader — place/amend/cancel orders, history
│   ├── position.rs     # PositionManager — leverage, margin, risk
│   ├── account.rs      # AccountManager — wallet, fee rates, borrow
│   ├── asset.rs        # AssetManager — transfers, deposits, settlements
│   ├── ws.rs           # Stream — WebSocket subscriptions & event loop
│   └── models/         # 150+ request/response model structs
│       ├── mod.rs
│       ├── linear_ticker/
│       ├── return_codes/
│       └── *.rs
└── tests/              # Integration tests
    ├── general_test.rs
    ├── market_test.rs
    ├── trade_test.rs
    ├── position_test.rs
    ├── account_test.rs
    ├── asset_test.rs
    ├── asset_comprehensive_test.rs
    └── ws_test.rs
```

### Manager Types

| Type | Constructor | Auth Required | Description |
|------|-------------|---------------|-------------|
| `General` | `Bybit::new(None, None)` | No | Server time, ping, system status |
| `MarketData` | `Bybit::new(None, None)` | No | Klines, order book, tickers, etc. |
| `Trader` | `Bybit::new(api_key, secret)` | Yes | Order management, history |
| `PositionManager` | `Bybit::new(api_key, secret)` | Yes | Position info, leverage, risk |
| `AccountManager` | `Bybit::new(api_key, secret)` | Yes | Wallet, fees, borrow |
| `AssetManager` | `Bybit::new(api_key, secret)` | Yes | Transfers, deposits, settlements |
| `Stream` | `Bybit::new(...)` | Varies | WebSocket subscriptions |

### The `Bybit` Trait

All manager types implement the `Bybit` trait:

```rust
pub trait Bybit {
    fn new(api_key: Option<String>, secret: Option<String>) -> Self;
    fn new_with_config(config: &Config, api_key: Option<String>, secret: Option<String>) -> Self;
}
```

---

## Examples

Full runnable examples are in the [`examples/`](examples/) directory:

| Example | Description |
|---------|-------------|
| [`rest_api.rs`](examples/rest_api.rs) | Demonstrates public and authenticated REST endpoints |
| [`websocket.rs`](examples/websocket.rs) | WebSocket public/private streams with multiple modes |
| [`new_trade_methods.rs`](examples/new_trade_methods.rs) | Advanced trade features: pre-check, borrow quota, DCP, batch orders |

Run them with:

```bash
# Public REST endpoints
cargo run --example rest_api

# With API keys
BYBIT_API_KEY="your_key" BYBIT_API_SECRET="your_secret" cargo run --example rest_api

# WebSocket — public trades
cargo run --example websocket trades

# WebSocket — order book
cargo run --example websocket orderbook

# WebSocket — private streams (requires keys)
BYBIT_API_KEY="key" BYBIT_API_SECRET="secret" cargo run --example websocket private

# Advanced trade methods
BYBIT_API_KEY="key" BYBIT_API_SECRET="secret" cargo run --example new_trade_methods
```

---

## Running Tests

```bash
# Compile tests (without running — requires live API keys)
cargo test --no-run

# Run specific test module
cargo test --test market_test -- --nocapture

# Run with logging
RUST_LOG=debug cargo test --test general_test -- --nocapture
```

> **Note**: Most integration tests require live API credentials and a network connection. Tests that hit authenticated endpoints will fail gracefully when credentials are empty.

---

## Development

### Pre-commit Hooks

This repository uses [pre-commit](https://pre-commit.com/) for code quality checks:

```bash
# Install the tool
pip install pre-commit   # or: brew install pre-commit

# Install hooks
pre-commit install
```

The pre-commit config runs:
- **typos** — Spell check
- **cargo fmt** — Rust formatting check

### CI

GitHub Actions runs `cargo test` on every push via [`.github/workflows/test.yml`](.github/workflows/test.yml).

---

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork** the repository and create a feature branch
2. **Follow existing patterns** — the codebase uses consistent patterns for requests, responses, and error handling
3. **Add tests** for new functionality
4. **Run `cargo fmt`** before committing
5. **Open a pull request** with a clear description of changes

### Code Style

- Follow the rules in `.pre-commit-config.yaml`
- Use `cargo clippy` to check for common mistakes
- Document all public APIs with doc comments

---

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

### Credit

This project draws inspiration from the design of [`binance-rs`](https://github.com/wisespace-io/binance-rs). Some parts may reflect similarities where both projects converge.

### Contributors

Special thanks to:

- [Sajjon](https://github.com/Sajjon) — For extensive contributions and code reviews
- [enseed](https://github.com/enseed-dev) — For valuable contributions and improvements

### Contact

For issues or questions, reach out on X (Twitter): [@unkuseni](https://twitter.com/unkuseni)