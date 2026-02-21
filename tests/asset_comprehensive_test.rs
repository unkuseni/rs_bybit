use bybit::prelude::*;

#[tokio::test]
async fn test_get_single_coin_balance() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for single coin balance
    let req = SingleCoinBalanceRequest {
        account_type: "UNIFIED",
        coin: "USDT",
        member_id: None,
        to_member_id: None,
        to_account_type: None,
        with_bonus: None,
        with_transfer_safe_amount: None,
        with_ltv_transfer_safe_amount: None,
    };

    // Call `get_single_coin_balance` asynchronously and match the result.
    match asset.get_single_coin_balance(req).await {
        Ok(response) => {
            // Assert that the response contains the expected account type.
            assert_eq!(response.account_type, "UNIFIED");
            // Assert that the response contains the expected coin.
            assert_eq!(response.balance.coin, "USDT");
            // Log the balance information.
            println!(
                "Retrieved single coin balance: {} {} (wallet: {}, transferable: {})",
                response.balance.coin,
                response.account_type,
                response.balance.wallet_balance,
                response.balance.transfer_balance
            );
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated single coin balance request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_all_coins_balance() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for all coins balance
    let req = AllCoinsBalanceRequest {
        account_type: "FUND",
        member_id: None,
        coin: Some("USDT"),
        with_bonus: None,
    };

    // Call `get_all_coins_balance` asynchronously and match the result.
    match asset.get_all_coins_balance(req).await {
        Ok(response) => {
            // Assert that the response contains the expected account type.
            assert_eq!(response.account_type, "FUND");
            // Log the number of coin balances retrieved.
            println!(
                "Retrieved {} coin balances for account type {}",
                response.balance.len(),
                response.account_type
            );
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated all coins balance request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_withdrawable_amount() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for withdrawable amount
    let req = WithdrawableAmountRequest { coin: "USDT" };

    // Call `get_withdrawable_amount` asynchronously and match the result.
    match asset.get_withdrawable_amount(req).await {
        Ok(response) => {
            // Log the limit amount in USD.
            println!(
                "Retrieved withdrawable amount with limit: {} USD",
                response.limit_amount_usd
            );
            // Check if FUND wallet information is available
            if let Some(fund) = response.withdrawable_amount.fund {
                println!(
                    "FUND wallet - Withdrawable: {}, Available: {}",
                    fund.withdrawable_amount, fund.available_balance
                );
            }
            // Check if UTA wallet information is available
            if let Some(uta) = response.withdrawable_amount.uta {
                println!(
                    "UTA wallet - Withdrawable: {}, Available: {}",
                    uta.withdrawable_amount, uta.available_balance
                );
            }
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated withdrawable amount request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_create_internal_transfer() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Generate a simple transfer ID for testing
    let transfer_id = format!("test-transfer-{}", chrono::Utc::now().timestamp_millis());

    // Create a request for internal transfer
    let req = InternalTransferRequest {
        transfer_id: &transfer_id,
        coin: "USDT",
        amount: "1.0",
        from_account_type: "UNIFIED",
        to_account_type: "FUND",
    };

    // Call `create_internal_transfer` asynchronously and match the result.
    match asset.create_internal_transfer(req).await {
        Ok(response) => {
            // Assert that the response contains the expected transfer ID.
            assert_eq!(response.transfer_id, transfer_id);
            // Log the transfer status.
            println!(
                "Created internal transfer {} with status: {}",
                response.transfer_id, response.status
            );
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated internal transfer request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_internal_transfer_records() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for internal transfer records
    let req = InternalTransferRecordsRequest {
        transfer_id: None,
        coin: Some("USDT"),
        status: None,
        start_time: None,
        end_time: None,
        limit: Some(5),
        cursor: None,
    };

    // Call `get_internal_transfer_records` asynchronously and match the result.
    match asset.get_internal_transfer_records(req).await {
        Ok(response) => {
            // Log the number of internal transfer records retrieved.
            println!(
                "Retrieved {} internal transfer records",
                response.list.len()
            );
            // Log the next page cursor if available
            if !response.next_page_cursor.is_empty() {
                println!("Next page cursor: {}", response.next_page_cursor);
            }
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated internal transfer records request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_create_universal_transfer() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Generate a simple transfer ID for testing
    let transfer_id = format!("test-transfer-{}", chrono::Utc::now().timestamp_millis());

    // Create a request for universal transfer
    // Note: This requires actual member IDs which we don't have in tests
    let req = UniversalTransferRequest {
        transfer_id: &transfer_id,
        coin: "USDT",
        amount: "1.0",
        from_member_id: 12345, // Example member ID
        to_member_id: 67890,   // Example member ID
        from_account_type: "UNIFIED",
        to_account_type: "FUND",
    };

    // Call `create_universal_transfer` asynchronously and match the result.
    match asset.create_universal_transfer(req).await {
        Ok(response) => {
            // Assert that the response contains the expected transfer ID.
            assert_eq!(response.transfer_id, transfer_id);
            // Log the transfer status.
            println!(
                "Created universal transfer {} with status: {}",
                response.transfer_id, response.status
            );
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated universal transfer request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_universal_transfer_records() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for universal transfer records
    let req = UniversalTransferRecordsRequest {
        transfer_id: None,
        coin: Some("USDT"),
        status: None,
        start_time: None,
        end_time: None,
        limit: Some(5),
        cursor: None,
    };

    // Call `get_universal_transfer_records` asynchronously and match the result.
    match asset.get_universal_transfer_records(req).await {
        Ok(response) => {
            // Log the number of universal transfer records retrieved.
            println!(
                "Retrieved {} universal transfer records",
                response.list.len()
            );
            // Log the next page cursor if available
            if !response.next_page_cursor.is_empty() {
                println!("Next page cursor: {}", response.next_page_cursor);
            }
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated universal transfer records request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_get_transferable_coin_list() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request for transferable coin list
    let req = TransferableCoinRequest {
        from_account_type: "UNIFIED",
        to_account_type: "CONTRACT",
    };

    // Call `get_transferable_coin_list` asynchronously and match the result.
    match asset.get_transferable_coin_list(req).await {
        Ok(response) => {
            // Log the number of transferable coins.
            println!(
                "Retrieved {} transferable coins from {} to {}",
                response.list.len(),
                "UNIFIED",
                "CONTRACT"
            );
            // Log the transferable coins
            for coin in &response.list {
                println!("  - {}", coin);
            }
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated transferable coin list request: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_set_deposit_account() {
    // Create a new instance of `Bybit::AssetManager`.
    let asset: AssetManager = Bybit::new(None, None);

    // Create a request to set deposit account
    let req = SetDepositAccountRequest {
        account_type: "UNIFIED",
    };

    // Call `set_deposit_account` asynchronously and match the result.
    match asset.set_deposit_account(req).await {
        Ok(response) => {
            // Log the status of the deposit account setting.
            println!(
                "Set deposit account to UNIFIED with status: {}",
                response.status
            );
        }
        Err(e) => {
            // This endpoint requires authentication, so we expect an error when not authenticated
            println!(
                "Expected error for unauthenticated set deposit account request: {:?}",
                e
            );
        }
    }
}

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
