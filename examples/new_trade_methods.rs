//! Advanced Trade Methods Example
//!
//! This example demonstrates the comprehensive trade features available in the Bybit V5 API.
//!
//! ## Prerequisites
//!
//! Set the following environment variables before running:
//!
//! ```bash
//! export BYBIT_API_KEY="your_api_key"
//! export BYBIT_SECRET="your_secret_key"
//! ```
//!
//! ## Running
//!
//! ```bash
//! cargo run --example new_trade_methods
//! ```

use bybit::prelude::*;
use std::{borrow::Cow, env};

#[tokio::main]
async fn main() -> Result<(), BybitError> {
    // Load API credentials from environment variables
    let api_key = env::var("BYBIT_API_KEY").ok();
    let secret = env::var("BYBIT_SECRET").ok();

    // Create a Trader instance (no credentials needed for some endpoints)
    let trader: Trader = Bybit::new(api_key.clone(), secret.clone());

    // ─────────────────────────────────────────────────────────
    // 1. Pre-check Order (Margin Calculation)
    //    Calculates the margin impact before placing an order.
    // ─────────────────────────────────────────────────────────
    println!("=== 1. Pre-check Order (Margin Calculation) ===");

    let pre_check_request = OrderRequest::custom(
        Category::Linear,
        "BTCUSDT",
        None,
        Side::Buy,
        OrderType::Limit,
        0.001,
        None,
        Some(50000.0),
        None,
        None,
        None,
        None,
        None,
        Some("GTC"),
        Some(0),
        Some("pre-check-example"),
        Some(55000.0),
        Some(48000.0),
        Some("LastPrice"),
        Some("LastPrice"),
        Some(false),
        Some(false),
        None,
        None,
        Some("Partial"),
        Some(54500.0),
        Some(48500.0),
        Some("Limit"),
        Some("Limit"),
        None,
        None,
        None,
        None,
    );

    match trader.pre_check_order(pre_check_request).await {
        Ok(response) => {
            println!(
                "Pre IMR: {:.4}%",
                response.result.pre_imr_e4 as f64 / 10000.0
            );
            println!(
                "Post IMR: {:.4}%",
                response.result.post_imr_e4 as f64 / 10000.0
            );
        }
        Err(e) => eprintln!("Pre-check order failed: {:?}", e),
    }

    // ─────────────────────────────────────────────────────────
    // 2. Get Borrow Quota (Spot Trading)
    //    Checks the maximum trade quantity and borrowable amount.
    // ─────────────────────────────────────────────────────────
    println!("\n=== 2. Get Borrow Quota (Spot Trading) ===");

    let borrow_request = BorrowQuotaRequest::new("BTCUSDT", Side::Buy);
    match trader.get_borrow_quota_spot(borrow_request).await {
        Ok(response) => {
            println!("Max Trade Qty: {}", response.result.max_trade_qty);
            println!("Borrow Coin: {}", response.result.borrow_coin);
        }
        Err(e) => eprintln!("Get borrow quota failed: {:?}", e),
    }

    // ─────────────────────────────────────────────────────────
    // 3. Configure Disconnection Protection (DCP)
    //    Sets up automatic order cancellation on disconnection.
    // ─────────────────────────────────────────────────────────
    println!("\n=== 3. Configure Disconnection Protection (DCP) ===");

    let dcp_request = DcpRequest::new(30, Some("DERIVATIVES"));
    match trader.set_dcp_options(dcp_request.clone()).await {
        Ok(response) => {
            println!(
                "DCP configured with {} second window",
                dcp_request.time_window
            );
            println!("DCP Response: {:?}", response);
        }
        Err(e) => eprintln!("DCP configuration failed: {:?}", e),
    }

    // ─────────────────────────────────────────────────────────
    // 4. Place a Custom Order with Advanced Features
    //    Demonstrates slippage tolerance, BBO settings, TP/SL.
    // ─────────────────────────────────────────────────────────
    println!("\n=== 4. Place Custom Order with Advanced Features ===");

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

    match trader.place_custom_order(order).await {
        Ok(response) => println!("Order placed: {:?}", response),
        Err(e) => eprintln!("Order placement failed: {:?}", e),
    }

    // ─────────────────────────────────────────────────────────
    // 5. Batch Place Orders
    //    Places multiple orders in a single API call.
    // ─────────────────────────────────────────────────────────
    println!("\n=== 5. Batch Place Orders ===");

    let orders = vec![
        OrderRequest {
            symbol: "ETHUSDT".into(),
            side: Side::Buy,
            qty: 100.0,
            order_type: OrderType::Market,
            ..Default::default()
        },
        OrderRequest {
            symbol: "BTCUSDT".into(),
            side: Side::Buy,
            qty: 100.0,
            order_type: OrderType::Market,
            ..Default::default()
        },
    ];
    let batch_request = BatchPlaceRequest::new(Category::Linear, orders);
    match trader.batch_place_order(batch_request).await {
        Ok(response) => println!("Batch order response: {:?}", response),
        Err(e) => eprintln!("Batch order failed: {:?}", e),
    }

    // ─────────────────────────────────────────────────────────
    // 6. Get Order History
    //    Retrieves historical orders with optional filters.
    // ─────────────────────────────────────────────────────────
    println!("\n=== 6. Get Order History ===");

    let history_request = OrderHistoryRequest::new(
        Category::Linear,
        None, // symbol
        None, // base_coin
        None, // settle_coin
        None, // order_id
        None, // order_link_id
        None, // order_filter
        None, // order_status
        None, // start_time
        None, // end_time
        None, // limit
    );

    match trader.get_order_history(history_request).await {
        Ok(response) => {
            println!(
                "Retrieved {} order history entries",
                response.result.list.len()
            );
        }
        Err(e) => eprintln!("Order history failed: {:?}", e),
    }

    // ─────────────────────────────────────────────────────────
    // 7. Get Trade History
    //    Retrieves execution records / trade fills.
    // ─────────────────────────────────────────────────────────
    println!("\n=== 7. Get Trade History ===");

    let trade_history_request = TradeHistoryRequest::new(
        Category::Linear,
        None, // symbol
        None, // order_id
        None, // order_link_id
        None, // base_coin
        None, // start_time
        None, // end_time
        None, // limit
        None, // cursor
    );

    match trader.get_trade_history(trade_history_request).await {
        Ok(response) => {
            println!(
                "Retrieved {} trade history entries",
                response.result.list.len()
            );
        }
        Err(e) => eprintln!("Trade history failed: {:?}", e),
    }

    Ok(())
}
