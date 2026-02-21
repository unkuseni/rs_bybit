use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a request to set Disconnection Protection (DCP) parameters on Bybit.
///
/// Disconnection Protection (DCP) automatically cancels all active orders if the client
/// remains disconnected from Bybit's WebSocket for longer than the specified time window.
/// This helps prevent unintended order execution during connection issues.
/// Bots should configure DCP based on their reconnection strategy and risk tolerance.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DcpRequest<'a> {
    /// Product type for which DCP applies.
    ///
    /// Specifies which product category's orders should be cancelled on disconnection:
    /// - `OPTIONS` (default): Options orders only
    /// - `DERIVATIVES`: Futures and perpetual orders (Inverse Perp, Inverse Futures,
    ///   USDT Perp, USDT Futures, USDC Perp, USDC Futures)
    /// - `SPOT`: Spot orders only
    ///
    /// Bots should set this based on which markets they are actively trading.
    /// The default is "OPTIONS" if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Cow<'a, str>>,

    /// Disconnection timing window in seconds.
    ///
    /// The time window (in seconds) after disconnection before orders are cancelled.
    /// Valid range: 3 to 300 seconds. Default is 10 seconds if not configured.
    /// Bots should set this based on their expected reconnection time and risk appetite.
    /// Shorter windows provide faster protection but may trigger unnecessarily during
    /// brief network interruptions.
    #[serde(rename = "timeWindow")]
    pub time_window: i32,
}

impl<'a> DcpRequest<'a> {
    /// Constructs a new DcpRequest with specified parameters.
    ///
    /// Creates a request to configure Disconnection Protection with a specific
    /// time window and optional product filter.
    ///
    /// # Arguments
    ///
    /// * `time_window` - Disconnection timing window in seconds (3-300)
    /// * `product` - Optional product type ("OPTIONS", "DERIVATIVES", or "SPOT")
    ///
    /// # Returns
    ///
    /// A new `DcpRequest` instance.
    ///
    /// # Panics
    ///
    /// This function does not panic, but Bybit API will reject time_window values
    /// outside the 3-300 range.
    pub fn new(time_window: i32, product: Option<&'a str>) -> Self {
        Self {
            product: product.map(Cow::Borrowed),
            time_window,
        }
    }

    /// Constructs a DcpRequest for derivatives with default 10-second window.
    ///
    /// Creates a request with 10-second time window for derivatives products.
    /// This is a common configuration for futures and perpetual trading bots.
    ///
    /// # Returns
    ///
    /// A `DcpRequest` with time_window=10 and product="DERIVATIVES".
    pub fn derivatives_default() -> Self {
        Self {
            product: Some(Cow::Borrowed("DERIVATIVES")),
            time_window: 10,
        }
    }

    /// Constructs a DcpRequest for spot with default 10-second window.
    ///
    /// Creates a request with 10-second time window for spot products.
    /// This is a common configuration for spot trading bots.
    ///
    /// # Returns
    ///
    /// A `DcpRequest` with time_window=10 and product="SPOT".
    pub fn spot_default() -> Self {
        Self {
            product: Some(Cow::Borrowed("SPOT")),
            time_window: 10,
        }
    }

    /// Constructs a DcpRequest for options with default 10-second window.
    ///
    /// Creates a request with 10-second time window for options products.
    /// This is the default configuration used by Bybit.
    ///
    /// # Returns
    ///
    /// A `DcpRequest` with time_window=10 and product="OPTIONS".
    pub fn options_default() -> Self {
        Self {
            product: Some(Cow::Borrowed("OPTIONS")),
            time_window: 10,
        }
    }

    /// Validates the DcpRequest parameters.
    ///
    /// Checks if the time_window is within the valid range (3-300 seconds).
    /// Bots should call this before sending the request to avoid API errors.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If parameters are valid
    /// * `Err(String)` - If time_window is outside valid range
    pub fn validate(&self) -> Result<(), String> {
        if self.time_window < 3 || self.time_window > 300 {
            return Err(format!(
                "time_window must be between 3 and 300 seconds, got {}",
                self.time_window
            ));
        }
        Ok(())
    }
}
