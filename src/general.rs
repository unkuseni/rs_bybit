use crate::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

/// The `General` struct provides general API functionality for the Bybit exchange.
///
/// This module includes methods for checking connectivity, retrieving server time,
/// and getting system status information. These endpoints are essential for
/// synchronizing trading operations with Bybit's servers and monitoring platform health.
impl General {
    /// Tests connectivity to the Bybit API server.
    ///
    /// This method sends a request to the server time endpoint to verify that
    /// the API is accessible and responsive. It's useful for health checks
    /// and connection monitoring in trading applications.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the server responds successfully, or `Ok(false)` if
    /// the server responds with an error. Returns `Err(BybitError)` for network
    /// or other critical failures.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let general = General::new(None, None);
    ///     match general.ping().await {
    ///         Ok(true) => println!("Connected to Bybit API"),
    ///         Ok(false) => println!("Bybit API responded with an error"),
    ///         Err(e) => println!("Connection failed: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn ping(&self) -> Result<bool, BybitError> {
        match self
            .client
            .get::<ServerTimeResponse>(API::Market(Market::Time), None)
            .await
        {
            Ok(_) => Ok(true),
            Err(BybitError::BybitError(content_error)) => {
                // Server responded but with an error code
                log::debug!(
                    "Server responded with error: code={}, msg={}",
                    content_error.code,
                    content_error.msg
                );
                Ok(false)
            }
            Err(e) => Err(e),
        }
    }

    /// Retrieves the current server time from Bybit with high precision.
    ///
    /// This endpoint returns the server time in seconds and nanoseconds since
    /// the Unix epoch. Accurate server time is critical for synchronizing
    /// trading bot operations and avoiding timestamp-related errors in API requests.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `ServerTimeResponse` if successful,
    /// or a `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let general = General::new(None, None);
    ///     match general.get_server_time().await {
    ///         Ok(response) => {
    ///             println!("Server time: {} seconds", response.result.time_second);
    ///             println!("Nanosecond component: {}", response.result.time_nano);
    ///         }
    ///         Err(e) => println!("Failed to get server time: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_server_time(&self) -> Result<ServerTimeResponse, BybitError> {
        self.client.get(API::Market(Market::Time), None).await
    }

    /// Retrieves the server time as a `chrono::DateTime<Utc>` object.
    ///
    /// This is a convenience method that converts the server time response
    /// into a more usable DateTime format for time calculations and comparisons.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `DateTime<Utc>` if successful,
    /// or a `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    /// use chrono::{DateTime, Utc};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let general = General::new(None, None);
    ///     match general.get_server_datetime().await {
    ///         Ok(dt) => println!("Server datetime: {}", dt),
    ///         Err(e) => println!("Failed to get server datetime: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_server_datetime(&self) -> Result<DateTime<Utc>, BybitError> {
        let response = self.get_server_time().await?;

        // Convert seconds to DateTime
        let secs = response.result.time_second as i64;
        let nanos = (response.result.time_nano % 1_000_000_000) as u32;

        DateTime::<Utc>::from_timestamp(secs, nanos)
            .ok_or_else(|| BybitError::Base("Invalid timestamp from server".to_string()))
    }

    /// Retrieves the server time in milliseconds since the Unix epoch.
    ///
    /// This is a convenience method that returns the server time in milliseconds,
    /// which is commonly used for API request timestamps and performance measurements.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing milliseconds as `u64` if successful,
    /// or a `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let general = General::new(None, None);
    ///     match general.get_server_time_millis().await {
    ///         Ok(ms) => println!("Server time in milliseconds: {}", ms),
    ///         Err(e) => println!("Failed to get server time: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_server_time_millis(&self) -> Result<u64, BybitError> {
        let response = self.get_server_time().await?;

        // Calculate milliseconds: seconds * 1000 + nanoseconds / 1_000_000
        let millis = response.result.time_second * 1000 + response.result.time_nano / 1_000_000;
        Ok(millis)
    }

    /// Retrieves the current timestamp for API requests.
    ///
    /// Bybit requires request timestamps to be within ±5 seconds of server time.
    /// This method returns the current server time in the format expected by
    /// Bybit API endpoints (milliseconds as string).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the timestamp as a `String` if successful,
    /// or a `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let general = General::new(None, None);
    ///     match general.get_api_timestamp().await {
    ///         Ok(timestamp) => println!("API timestamp: {}", timestamp),
    ///         Err(e) => println!("Failed to get API timestamp: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_api_timestamp(&self) -> Result<String, BybitError> {
        let millis = self.get_server_time_millis().await?;
        Ok(millis.to_string())
    }

    /// Retrieves the system status from the Bybit API.
    ///
    /// This endpoint returns information about platform maintenance or service incidents.
    /// It's useful for monitoring platform health and scheduling maintenance windows.
    ///
    /// # Parameters
    ///
    /// * `id` - Optional unique identifier for filtering system status records
    /// * `state` - Optional system state for filtering (e.g., "completed", "in_progress", "scheduled")
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `SystemStatusResponse` if successful,
    /// or a `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bybit::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let general = General::new(None, None);
    ///
    ///     // Get all system status records
    ///     match general.get_system_status(None, None).await {
    ///         Ok(response) => println!("System status: {:?}", response),
    ///         Err(e) => println!("Failed to get system status: {:?}", e),
    ///     }
    ///
    ///     // Get only completed incidents
    ///     match general.get_system_status(None, Some("completed".to_string())).await {
    ///         Ok(response) => println!("Completed incidents: {:?}", response),
    ///         Err(e) => println!("Failed to get completed incidents: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_system_status(
        &self,
        id: Option<String>,
        state: Option<String>,
    ) -> Result<SystemStatusResponse, BybitError> {
        // Build query parameters
        let mut params = BTreeMap::new();
        if let Some(id_value) = id {
            params.insert("id".to_string(), id_value);
        }
        if let Some(state_value) = state {
            params.insert("state".to_string(), state_value);
        }

        // Build request string from parameters
        let request = build_request(&params);

        // Call the get method on the client field of self
        self.client
            .get(API::Market(Market::SystemStatus), Some(request))
            .await
    }
}
