use crate::prelude::*;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

/// The `General` struct represents general functionality for the Bybit API.
impl General {
    /// Tests for connectivity by sending a ping request to the Bybit server.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `String` with the response message if successful,

    /// or a `BybitError` if an error occurs.
    pub async fn ping(&self) -> Result<String, BybitError> {
        // Call the get method on the client field of self, passing in the time variable and None as arguments, and return the result
        let _response: ServerTimeResponse =
            self.client.get(API::Market(Market::Time), None).await?;

        // prints pong to the console
        Ok("pong: Hi, this is bybit".to_string())
    }

    /// Retrieves the server time from the Bybit API.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `ServerTime` struct if successful,

    /// or a `BybitError` if an error occurs.
    pub async fn get_server_time(&self) -> Result<ServerTimeResponse, BybitError> {
        // Create a variable called time and set it to an API::Market enum variant with a Market::Time value
        // Call the get method on the client field of self, passing in the time variable and None as arguments, and return the result
        let response: ServerTimeResponse = self.client.get(API::Market(Market::Time), None).await?;

        // Return the ServerTime struct
        Ok(response)
    }

    /// Retrieves the system status from the Bybit API.
    ///
    /// This endpoint returns information about platform maintenance or service incidents.
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
        let response: SystemStatusResponse = self
            .client
            .get(API::Market(Market::SystemStatus), Some(request))
            .await?;

        Ok(response)
    }
}
