use crate::prelude::*;

/// Parameters for confirming new risk limit (MMR - Maintenance Margin Rate).
///
/// Used to construct a request to the `/v5/position/confirm-pending-mmr` endpoint to confirm
/// a new risk limit when a position is marked as reduce-only. Bots use this to remove the
/// reduce-only restriction after adjusting risk parameters to comply with new risk limits.
///
/// # Bybit API Reference
/// According to the Bybit V5 API documentation:
/// - This endpoint is only applicable when the user is marked as only reducing positions
///   (see the `isReduceOnly` field in the Get Position Info interface).
/// - After the user actively adjusts the risk level, this interface is called to try to
///   calculate the adjusted risk level.
/// - If the call passes (retCode=0), the system will remove the position reduceOnly mark.
/// - It's recommended to call Get Position Info to check the `isReduceOnly` field before
///   and after calling this endpoint.
///
/// # Usage Example
/// ```rust
/// // Confirm new risk limit for a position marked as reduce-only
/// let request = ConfirmPendingMmrRequest::new(Category::Linear, "BTCUSDT");
///
/// // After adjusting leverage, risk limit, or adding margin
/// // Call this endpoint to remove the reduce-only restriction
/// ```
#[derive(Clone)]
pub struct ConfirmPendingMmrRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Must match the category of the position that is
    /// marked as reduce-only. Bots must set this correctly to target the affected position.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the specific position that is marked as reduce-only. Bots must specify
    /// the exact symbol to confirm the risk limit for the correct position.
    pub symbol: Cow<'a, str>,
}

impl<'a> ConfirmPendingMmrRequest<'a> {
    /// Constructs a new ConfirmPendingMmr request with specified parameters.
    ///
    /// Allows customization of the confirm pending MMR request. Bots should use this
    /// to specify the exact symbol and category of the reduce-only position.
    ///
    /// # Arguments
    /// * `category` - The product category (Linear, Inverse)
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    pub fn new(category: Category, symbol: &'a str) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
        }
    }

    /// Creates a default ConfirmPendingMmr request.
    ///
    /// Returns a request with `category` set to `Linear` and `symbol` set to `"BTCUSDT"`.
    /// Suitable for testing but should be customized for production to match the actual
    /// reduce-only position.
    pub fn default() -> ConfirmPendingMmrRequest<'a> {
        ConfirmPendingMmrRequest::new(Category::Linear, "BTCUSDT")
    }
}

impl<'a> Default for ConfirmPendingMmrRequest<'a> {
    fn default() -> Self {
        Self::new(Category::Linear, "BTCUSDT")
    }
}

impl<'a> ConfirmPendingMmrRequest<'a> {
    /// Validates the request parameters.
    ///
    /// Bots should call this method before sending the request to ensure basic validation.
    /// Note: The primary validation happens on Bybit's side based on position state.
    ///
    /// # Returns
    /// * `Ok(())` if validation passes
    /// * `Err(String)` with error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        // Validate category is supported for this endpoint
        match self.category {
            Category::Linear | Category::Inverse => Ok(()),
            _ => Err("Category must be 'linear' or 'inverse' for confirm pending MMR".to_string()),
        }
    }

    /// Sets the symbol for the request.
    ///
    /// Convenience method for updating the symbol.
    ///
    /// # Arguments
    /// * `symbol` - The trading pair symbol
    pub fn with_symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Cow::Borrowed(symbol);
        self
    }

    /// Sets the category for the request.
    ///
    /// Convenience method for updating the category.
    ///
    /// # Arguments
    /// * `category` - The product category
    pub fn with_category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let request = ConfirmPendingMmrRequest::new(Category::Linear, "BTCUSDT");
        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
    }

    #[test]
    fn test_default() {
        let request = ConfirmPendingMmrRequest::default();
        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
    }

    #[test]
    fn test_validation() {
        // Valid requests
        let linear_request = ConfirmPendingMmrRequest::new(Category::Linear, "BTCUSDT");
        assert!(linear_request.validate().is_ok());

        let inverse_request = ConfirmPendingMmrRequest::new(Category::Inverse, "BTCUSD");
        assert!(inverse_request.validate().is_ok());

        // Invalid category (option is not supported for this endpoint)
        let option_request = ConfirmPendingMmrRequest::new(Category::Option, "BTC-OPTION");
        assert!(option_request.validate().is_err());
    }

    #[test]
    fn test_builder_methods() {
        let request = ConfirmPendingMmrRequest::default()
            .with_category(Category::Inverse)
            .with_symbol("ETHUSD");

        assert_eq!(request.category, Category::Inverse);
        assert_eq!(request.symbol, "ETHUSD");
    }

    #[test]
    fn test_clone() {
        let request1 = ConfirmPendingMmrRequest::new(Category::Linear, "BTCUSDT");
        let request2 = request1.clone();

        assert_eq!(request1.category, request2.category);
        assert_eq!(request1.symbol, request2.symbol);
    }

    #[test]
    fn test_default_impl() {
        let request: ConfirmPendingMmrRequest = Default::default();
        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
    }
}
