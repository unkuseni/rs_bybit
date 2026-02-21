use bybit::prelude::*;

mod tests {
    use super::*;
    use tokio::test;

    static API_KEY: &str = ""; // Mockup string
    static SECRET: &str = ""; // Mockup string

    #[test]
    async fn position_info() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = PositionRequest::new(Category::Linear, Some("BTCUSDT"), None, None, None);
        match position.get_info(request).await {
            Ok(data) => println!("Position Info: {:#?}", data),
            Err(e) => println!("Position Info Error: {:#?}", e),
        }
    }

    #[test]
    async fn set_leverage_symmetric() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "10");
        match position.set_leverage(request).await {
            Ok(data) => println!("Set Leverage (Symmetric): {:?}", data),
            Err(e) => println!("Set Leverage Error: {:?}", e),
        }
    }

    #[test]
    async fn set_leverage_asymmetric() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = LeverageRequest::new(Category::Linear, "BTCUSDT", "15", "8");
        match position.set_leverage(request).await {
            Ok(data) => println!("Set Leverage (Asymmetric): {:?}", data),
            Err(e) => println!("Set Leverage Error: {:?}", e),
        }
    }

    #[test]
    async fn set_margin_mode() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = ChangeMarginRequest::new(Category::Linear, "BTCUSDT", 1, 10);
        match position.set_margin_mode(request).await {
            Ok(data) => println!("Set Margin Mode: {:?}", data),
            Err(e) => println!("Set Margin Mode Error: {:?}", e),
        }
    }

    #[test]
    async fn set_position_mode() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = MarginModeRequest::new(Category::Linear, 0, Some("BTCUSDT"), None);
        match position.set_position_mode(request).await {
            Ok(data) => println!("Set Position Mode: {:?}", data),
            Err(e) => println!("Set Position Mode Error: {:?}", e),
        }
    }

    #[test]
    async fn set_risk_limit() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = SetRiskLimit::new(Category::Linear, "BTCUSDT", 1, None);
        match position.set_risk_limit(request).await {
            Ok(data) => println!("Set Risk Limit: {:?}", data),
            Err(e) => println!("Set Risk Limit Error: {:?}", e),
        }
    }

    #[test]
    async fn set_trading_stop_full() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            "Full",
            0,
            Some(50000.0),
            Some(45000.0),
            Some(100.0),
            Some("MarkPrice"),
            Some("MarkPrice"),
            Some(51000.0),
            None,
            None,
            None,
            None,
            Some(OrderType::Market),
            Some(OrderType::Market),
        );
        match position.set_trading_stop(request).await {
            Ok(data) => println!("Set Trading Stop (Full): {:?}", data),
            Err(e) => println!("Set Trading Stop Error: {:?}", e),
        }
    }

    #[test]
    async fn set_trading_stop_partial() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            "Partial",
            0,
            Some(50000.0),
            None,
            None,
            Some("MarkPrice"),
            None,
            None,
            Some(0.5),
            Some(0.5),
            None,
            None,
            Some(OrderType::Market),
            Some(OrderType::Market),
        );
        match position.set_trading_stop(request).await {
            Ok(data) => println!("Set Trading Stop (Partial): {:?}", data),
            Err(e) => println!("Set Trading Stop Error: {:?}", e),
        }
    }

    #[test]
    async fn set_add_margin() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = AddMarginRequest::new(Category::Linear, "BTCUSDT", false, None);
        match position.set_add_margin(request).await {
            Ok(data) => println!("Set Add Margin: {:?}", data),
            Err(e) => println!("Set Add Margin Error: {:?}", e),
        }
    }

    #[test]
    async fn add_or_reduce_margin() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = AddReduceMarginRequest::new(Category::Linear, "BTCUSDT", 50.0, None);
        match position.add_or_reduce_margin(request).await {
            Ok(data) => println!("Add/Reduce Margin: {:?}", data),
            Err(e) => println!("Add/Reduce Margin Error: {:?}", e),
        }
    }

    #[test]
    async fn get_closed_pnl() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = ClosedPnlRequest::new(Category::Linear, Some("BTCUSDT"), None, None, None);
        match position.get_closed_pnl(request).await {
            Ok(data) => println!("Get Closed PnL: {:#?}", data),
            Err(e) => println!("Get Closed PnL Error: {:#?}", e),
        }
    }

    #[test]
    async fn get_closed_options_positions() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = ClosedOptionsPositionsRequest::new(
            Category::Option,
            Some("BTC-12JUN25-104019-C-USDT"),
            None,
            None,
            Some(50),
            None,
        );
        match position.get_closed_options_positions(request).await {
            Ok(data) => println!("Get Closed Options Positions: {:#?}", data),
            Err(e) => println!("Get Closed Options Positions Error: {:#?}", e),
        }
    }

    #[test]
    async fn get_closed_options_positions_default() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = ClosedOptionsPositionsRequest::default();
        match position.get_closed_options_positions(request).await {
            Ok(data) => println!("Get Closed Options Positions (Default): {:#?}", data),
            Err(e) => println!("Get Closed Options Positions Error: {:#?}", e),
        }
    }

    #[test]
    async fn confirm_pending_mmr() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = ConfirmPendingMmrRequest::new(Category::Linear, "BTCUSDT");
        match position.confirm_pending_mmr(request).await {
            Ok(data) => println!("Confirm Pending MMR: {:?}", data),
            Err(e) => println!("Confirm Pending MMR Error: {:?}", e),
        }
    }

    #[test]
    async fn move_position() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        // Note: MovePositionRequest requires UIDs which we don't have in test environment
        // This test will likely fail but demonstrates the API usage
        let request = MovePositionRequest::default();
        match position.move_position(request).await {
            Ok(data) => println!("Move Position: {:?}", data),
            Err(e) => println!("Move Position Error: {:?}", e),
        }
    }

    #[test]
    async fn move_position_history() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request =
            MoveHistoryRequest::new(Some(Category::Linear), None, None, None, None, None, None);
        match position.move_position_history(request).await {
            Ok(data) => println!("Move Position History: {:#?}", data),
            Err(e) => println!("Move Position History Error: {:#?}", e),
        }
    }
}

// Separate module for unit tests (non-async)
mod unit_tests {
    use super::*;

    #[test]
    fn test_leverage_request_validation() {
        // Test valid symmetric leverage
        let valid_symmetric = LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "10");
        assert!(valid_symmetric.validate().is_ok());

        // Test valid asymmetric leverage
        let valid_asymmetric = LeverageRequest::new(Category::Linear, "BTCUSDT", "15", "8");
        assert!(valid_asymmetric.validate().is_ok());

        // Test invalid leverage (too low)
        let invalid_low = LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "0");
        assert!(invalid_low.validate().is_err());

        // Test invalid leverage (too high)
        let invalid_high = LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "201");
        assert!(invalid_high.validate().is_err());
    }

    #[test]
    fn test_trading_stop_request_validation() {
        // Test valid full trading stop
        let valid_full = TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            "Full",
            0,
            Some(50000.0),
            Some(45000.0),
            Some(100.0),
            Some("MarkPrice"),
            Some("MarkPrice"),
            Some(51000.0),
            None,
            None,
            None,
            None,
            Some(OrderType::Market),
            Some(OrderType::Market),
        );
        assert!(valid_full.validate().is_ok());

        // Test valid partial trading stop
        let valid_partial = TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            "Partial",
            0,
            Some(50000.0),
            None,
            None,
            Some("MarkPrice"),
            None,
            None,
            Some(0.5),
            Some(0.5),
            None,
            None,
            Some(OrderType::Market),
            Some(OrderType::Market),
        );
        assert!(valid_partial.validate().is_ok());

        // Test invalid: negative stop loss
        let invalid_sl = TradingStopRequest::new(
            Category::Linear,
            "BTCUSDT",
            "Full",
            0,
            Some(50000.0),
            Some(-100.0), // Negative stop loss (invalid)
            Some(100.0),
            Some("MarkPrice"),
            Some("MarkPrice"),
            Some(51000.0),
            None,
            None,
            None,
            None,
            Some(OrderType::Market),
            Some(OrderType::Market),
        );
        assert!(invalid_sl.validate().is_err());
    }

    #[test]
    fn test_closed_options_positions_request_validation() {
        // Test valid request
        let valid = ClosedOptionsPositionsRequest::new(
            Category::Option,
            Some("BTC-12JUN25-104019-C-USDT"),
            Some(1749730000000),
            Some(1749736000000),
            Some(25),
            Some("cursor_token"),
        );
        assert!(valid.validate().is_ok());

        // Test invalid category
        let invalid_category = ClosedOptionsPositionsRequest::new(
            Category::Linear, // Wrong category
            Some("BTC-12JUN25-104019-C-USDT"),
            Some(1749730000000),
            Some(1749736000000),
            Some(25),
            Some("cursor_token"),
        );
        assert!(invalid_category.validate().is_err());

        // Test invalid time range (start >= end)
        let invalid_time = ClosedOptionsPositionsRequest::new(
            Category::Option,
            Some("BTC-12JUN25-104019-C-USDT"),
            Some(1749736000000),
            Some(1749730000000), // End before start
            Some(25),
            Some("cursor_token"),
        );
        assert!(invalid_time.validate().is_err());
    }

    #[test]
    fn test_confirm_pending_mmr_request_validation() {
        // Test valid request
        let valid = ConfirmPendingMmrRequest::new(Category::Linear, "BTCUSDT");
        assert!(valid.validate().is_ok());

        // Test invalid category (not linear or inverse)
        let invalid_category = ConfirmPendingMmrRequest::new(Category::Option, "BTCUSDT");
        assert!(invalid_category.validate().is_err());
    }

    // Test model builder pattern usage
    #[test]
    fn test_closed_options_positions_request_builder() {
        let request = ClosedOptionsPositionsRequest::default()
            .with_symbol("BTC-12JUN25-104019-C-USDT")
            .with_time_range(1749730000000, 1749736000000)
            .with_limit(25)
            .with_cursor("cursor_token");

        assert_eq!(request.category, Category::Option);
        assert_eq!(request.symbol.unwrap(), "BTC-12JUN25-104019-C-USDT");
        assert_eq!(request.start_time.unwrap(), 1749730000000);
        assert_eq!(request.end_time.unwrap(), 1749736000000);
        assert_eq!(request.limit.unwrap(), 25);
        assert_eq!(request.cursor.unwrap(), "cursor_token");
    }

    // Test error handling
    #[test]
    fn test_position_error_handling() {
        // Create a position manager with invalid credentials
        let position: PositionManager =
            Bybit::new(Some("invalid_key".into()), Some("invalid_secret".into()));

        // This should fail with authentication error when actually called
        // We're just testing that the struct can be created
        assert!(!position.client.api_key.is_empty());
    }
}
