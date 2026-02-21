use bybit::prelude::*;

/// Module containing unit tests for the Bybit API.
mod tests {
    // Import the parent module.
    use super::*;
    // Import the tokio test module.
    use tokio::test;

    /// Test case that checks the functionality of the `get_server_time` and `ping`
    /// methods of the `Bybit::General` struct.
    ///
    /// This test case creates a new instance of `Bybit::General`, calls
    /// `get_server_time` and `ping` asynchronously, and prints the result or
    /// error.
    #[test]
    async fn test_time() {
        // Create a new instance of `Bybit::General`.
        let general: General = Bybit::new(None, None);

        // Call `get_server_time` asynchronously and match the result.
        match general.get_server_time().await {
            // If the call is successful, print the data.
            Ok(data) => println!("{:#?}", data),
            // If the call fails, print the error.
            Err(err) => println!("{:#?}", err),
        }
    }

    #[test]
    async fn test_ping() {
        let general: General = Bybit::new(None, None);
        // Call `ping` asynchronously and match the result.
        match general.ping().await {
            // If the call is successful, print the connection status.
            Ok(true) => println!("Connected to Bybit API"),
            Ok(false) => println!("Bybit API responded with an error"),
            // If the call fails, print the error.
            Err(err) => println!("{:#?}", err),
        }
    }

    /// Test case that checks the functionality of the `get_system_status`
    /// method of the `Bybit::General` struct.
    ///
    /// This test case creates a new instance of `Bybit::General`, calls
    /// `get_system_status` asynchronously with optional filters, and prints
    /// the result or error.
    #[test]
    async fn test_system_status() {
        // Create a new instance of `Bybit::General`.
        let general: General = Bybit::new(None, None);

        // Call `get_system_status` asynchronously without filters
        println!("Testing get_system_status without filters:");
        match general.get_system_status(None, None).await {
            // If the call is successful, print the data.
            Ok(data) => println!("{:#?}", data),
            // If the call fails, print the error.
            Err(err) => println!("{:#?}", err),
        }

        // Call `get_system_status` asynchronously with state filter
        println!("\nTesting get_system_status with state filter:");
        match general
            .get_system_status(None, Some("completed".to_string()))
            .await
        {
            // If the call is successful, print the data.
            Ok(data) => println!("{:#?}", data),
            // If the call fails, print the error.
            Err(err) => println!("{:#?}", err),
        }
    }
}
