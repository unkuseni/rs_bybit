# Bybit API v5 client in Rust

This is simply a bybit V5 api connector using binance-rs implementation, Some parts of the api have not been completed yet...Anyone is welcome to branch/fork the repository and add their own upgrades, see [Development](development) section for details.

> [!CAUTION]
> This is a personal project, use at your own risk. Neither the original author,
> nor any of the contributors of this software shall ever be held responsible
> for your investment losses. Cryptocurrency investment is subject to high market risk.

# Table of Contents

- [Description](#description)
- [Features](#features)
- [Development](#development)
- [Usage](#usage)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)

# Features

Some part of the project is still under development. Please regularly take a look at this README for updates.

- **REST API:**
  - [x] **Market Data:** Access to K-line, tickers, order book, and more. See [`tests/market_test.rs`](https://github.com/unkuseni/rs_bybit/tests/market_test.rs)
  - [x] **Trade:** Functionality for placing, amending, and canceling orders. See [`tests/trade_test.rs`](https://github.com/unkuseni/rs_bybit/tests/trade_test.rs)
    - [x] **Order Management:** Place, amend, cancel orders (single and batch)
    - [x] **Order History:** Query open, closed, and historical orders
    - [x] **Trade History:** Get execution records
    - [x] **Pre-check Orders:** Calculate margin impact before placement
    - [x] **Borrow Quota:** Check available balance for spot and margin trading
    - [x] **Disconnection Protection (DCP):** Configure automatic order cancellation on disconnection
    - [x] **Advanced Order Features:** Slippage tolerance, BBO (Best Bid/Offer) settings, TP/SL modes
  - [x] **Position:** Manage your trading positions. See [`tests/position_test.rs`](https://github.com/unkuseni/rs_bybit/tests/position_test.rs)
  - [ ] **Account & Asset:** These sections are currently under active development. See [`tests/account_test.rs`](https://github.com/unkuseni/rs_bybit/tests/account_test.rs) for progress
- **Websocket API:**
  - [x] Support for subscribing to real-time public and private data streams. See [`tests/ws_test.rs`](https://github.com/unkuseni/rs_bybit/tests/ws_test.rs)

# Development

If you want to contribubute please make sure to follow this setup. Install the precommit tool if you don't have it installed already and make sure to install the pre-commit hooks

## Precommit

Install the [`pre-commit` CLI tool](https://pre-commit.com/) and in this repo install the hooks.

### Install tool

```sh
brew install pre-commit
```

### Install hooks

```sh
pre-commit install
```

# Usage

This crate can be installed by adding the following to your `Cargo.toml`:

```toml
[dependencies]
rs_bybit = "*"
```

Take a look at tests for usage.

## New Trade Methods Examples

The library now includes comprehensive support for Bybit's V5 trade API. Here are examples of the newly implemented features:

### 1. Pre-check Order (Margin Calculation)

```rust
let pre_check_request = OrderRequest::custom(
    Category::Linear,
    "BTCUSDT",
    None,
    Side::Buy,
    OrderType::Limit,
    0.001,
    None,
    Some(50000.0),
    None, None, None, None, None,
    Some("GTC"),
    Some(0),
    Some("pre-check-example"),
    Some(55000.0),
    Some(48000.0),
    Some("LastPrice"),
    Some("LastPrice"),
    Some(false),
    Some(false),
    None, None,
    Some("Partial"),
    Some(54500.0),
    Some(48500.0),
    Some("Limit"),
    Some("Limit"),
    None, None, None, None,
);

let response = trader.pre_check_order(pre_check_request).await?;
println!("Pre IMR: {:.4}%", response.result.pre_imr_e4 as f64 / 10000.0);
println!("Post IMR: {:.4}%", response.result.post_imr_e4 as f64 / 10000.0);
```

### 2. Get Borrow Quota (Spot Trading)

```rust
let borrow_request = BorrowQuotaRequest::new("BTCUSDT", Side::Buy);
let response = trader.get_borrow_quota_spot(borrow_request).await?;
println!("Max Trade Qty: {}", response.result.max_trade_qty);
println!("Borrow Coin: {}", response.result.borrow_coin);
```

### 3. Configure Disconnection Protection (DCP)

```rust
let dcp_request = DcpRequest::new(30, Some("DERIVATIVES"));
let response = trader.set_dcp_options(dcp_request).await?;
println!("DCP configured with {} second window", dcp_request.time_window);
```

### 4. Advanced Order Features

```rust
let order = OrderRequest {
    category: Category::Linear,
    symbol: Cow::Borrowed("ETHUSDT"),
    side: Side::Buy,
    order_type: OrderType::Market,
    qty: 0.1,
    slippage_tolerance_type: Some(Cow::Borrowed("Percent")),
    slippage_tolerance: Some(0.5), // 0.5% slippage tolerance
    bbo_side_type: Some(Cow::Borrowed("Queue")),
    bbo_level: Some(1),
    ..OrderRequest::default()
};
```

For complete examples, see [`examples/new_trade_methods.rs`](examples/new_trade_methods.rs).

# Contact

if you have any issues contact me on X (twitter) @unkuseni

# Acknowledgments

## Credit

I like the project design of binance-rs and decided to use it. You might stumble upon some changes where both projects differ.

## Special thanks

A special thank you to [Sajjon](https://github.com/Sajjon) for all of his many [contributions](https://github.com/unkuseni/rs_bybit/pulls?q=is%3Amerged+is%3Apr+author%3Asajjon+).

Also thanks to [enseed](https://github.com/enseed-dev) for the [contributions](https://github.com/unkuseni/rs_bybit/pulls?q=is%3Amerged+is%3Apr+author%3Aenseed-dev+).
