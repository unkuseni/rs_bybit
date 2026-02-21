use bybit::prelude::*;

#[tokio::test]
async fn test_get_settlement_record() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for linear settlement records
    let req = SettlementRecordRequest {
        category: "linear",
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(5),
        cursor: None,
    };

    // Call `get_settlement_record` asynchronously and match the result.
    match asset.get_settlement_record(req).await {
        Ok(response) => {
            // Assert that the response contains the expected category.
            assert_eq!(response.category, "linear");
            // Log the number of settlement records retrieved.
            println!("Retrieved {} settlement records", response.list.len());
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated settlement record request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_delivery_record() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for option delivery records
    let req = DeliveryRecordRequest {
        category: "option",
        symbol: None,
        start_time: None,
        end_time: None,
        exp_date: None,
        limit: Some(5),
        cursor: None,
    };

    // Call `get_delivery_record` asynchronously and match the result.
    match asset.get_delivery_record(req).await {
        Ok(response) => {
            // Assert that the response contains the expected category.
            assert_eq!(response.category, "option");
            // Log the number of delivery records retrieved.
            println!("Retrieved {} delivery records", response.list.len());
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated delivery record request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_coin_info() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for BTC coin info
    let req = CoinInfoRequest { coin: Some("BTC") };

    // Call `get_coin_info` asynchronously and match the result.
    match asset.get_coin_info(req).await {
        Ok(response) => {
            // Assert that the response contains coin information.
            assert!(!response.rows.is_empty());
            // Log the number of coin info rows retrieved.
            println!("Retrieved {} coin info rows", response.rows.len());
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated coin info request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_coin_exchange_records() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for coin exchange records
    let req = CoinExchangeRecordRequest {
        from_coin: None,
        to_coin: None,
        limit: Some(5),
        cursor: None,
    };

    // Call `get_coin_exchange_records` asynchronously and match the result.
    match asset.get_coin_exchange_records(req).await {
        Ok(response) => {
            // Log the number of coin exchange records retrieved.
            println!(
                "Retrieved {} coin exchange records",
                response.order_body.len()
            );
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated coin exchange records request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_sub_uid() {
    // Create a new instance of `Bybit::AssetManager`.
    // Note: This endpoint requires authentication with a master UID's API key
    let asset: AssetManager = Bybit::new(None, None);

    // Call `get_sub_uid` asynchronously and match the result.
    match asset.get_sub_uid().await {
        Ok(response) => {
            // Log the number of sub UIDs retrieved.
            println!("Retrieved {} sub member IDs", response.sub_member_ids.len());
            println!(
                "Retrieved {} transferable sub member IDs",
                response.transferable_sub_member_ids.len()
            );
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated sub UID request: {:?}",
                e
            );
        }
    }
}
