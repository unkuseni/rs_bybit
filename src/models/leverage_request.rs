use crate::prelude::*;

/// Parameters for setting leverage for a trading pair.
///
/// Used to construct a request to the `/v5/position/set-leverage` endpoint to adjust leverage for a specific symbol.
/// Bots use this to manage risk exposure in perpetual futures, as higher leverage increases both potential returns and liquidation risk.
///
/// # Bybit API Reference
/// According to the Bybit V5 API documentation:
/// - For one-way mode: `buyLeverage` must be the same as `sellLeverage`
/// - For hedge mode with isolated margin: `buyLeverage` and `sellLeverage` can be different
/// - For hedge mode with cross margin: `buyLeverage` must be the same as `sellLeverage`
///
/// # Usage Example
/// ```rust
/// // For one-way mode or cross margin hedge mode (same leverage for both sides)
/// let request = LeverageRequest::new(Category::Linear, "BTCUSDT", 10, 10);
///
/// // For hedge mode with isolated margin (different leverage for buy and sell)
/// let request = LeverageRequest::new(Category::Linear, "BTCUSDT", 15, 8);
/// ```
#[derive(Clone, Default)]
pub struct LeverageRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for which leverage is being set. Bots must specify a valid symbol.
    pub symbol: Cow<'a, str>,

    /// The leverage for buy positions (e.g., 10 for 10x).
    ///
    /// The desired leverage multiplier for long positions. Bots should ensure this complies with Bybit's maximum leverage limits for the symbol to avoid request failures.
    pub buy_leverage: String,

    /// The leverage for sell positions (e.g., 10 for 10x).
    ///
    /// The desired leverage multiplier for short positions. For one-way mode or cross margin hedge mode, this must equal `buy_leverage`.
    pub sell_leverage: String,
}

impl<'a> LeverageRequest<'a> {
    /// Constructs a new Leverage request with specified parameters.
    ///
    /// Allows customization of the leverage request. Bots should use this to specify the exact symbol, category, and leverage values.
    ///
    /// # Arguments
    /// * `category` - The product category (Linear, Inverse)
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    /// * `buy_leverage` - The leverage for buy positions (as a string, e.g., "10")
    /// * `sell_leverage` - The leverage for sell positions (as a string, e.g., "10")
    pub fn new(
        category: Category,
        symbol: &'a str,
        buy_leverage: &str,
        sell_leverage: &str,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            buy_leverage: buy_leverage.to_string(),
            sell_leverage: sell_leverage.to_string(),
        }
    }

    /// Constructs a new Leverage request with the same leverage for both buy and sell sides.
    ///
    /// Convenience method for one-way mode or cross margin hedge mode where buy and sell leverage must be equal.
    ///
    /// # Arguments
    /// * `category` - The product category (Linear, Inverse)
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    /// * `leverage` - The leverage for both buy and sell positions (as a string, e.g., "10")
    pub fn new_symmetric(category: Category, symbol: &'a str, leverage: &str) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            buy_leverage: leverage.to_string(),
            sell_leverage: leverage.to_string(),
        }
    }

    /// Creates a default Leverage request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, and both `buy_leverage` and `sell_leverage` set to `"10"`.
    /// Suitable for testing but should be customized for production.
    pub fn default() -> LeverageRequest<'a> {
        LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "10")
    }

    /// Validates that the leverage values are within acceptable ranges.
    ///
    /// Bots should call this method before sending the request to ensure compliance with Bybit's leverage limits.
    /// Note: This is a basic validation; actual limits depend on the symbol and risk tier.
    pub fn validate(&self) -> Result<(), String> {
        // Parse leverage values
        let buy_leverage: f64 = self
            .buy_leverage
            .parse()
            .map_err(|_| "Invalid buy_leverage format")?;
        let sell_leverage: f64 = self
            .sell_leverage
            .parse()
            .map_err(|_| "Invalid sell_leverage format")?;

        // Check minimum leverage (must be at least 1)
        if buy_leverage < 1.0 {
            return Err("buy_leverage must be at least 1".to_string());
        }
        if sell_leverage < 1.0 {
            return Err("sell_leverage must be at least 1".to_string());
        }

        // Check that leverage values are reasonable (Bybit typically has max leverage around 100-125x)
        if buy_leverage > 200.0 {
            return Err("buy_leverage exceeds reasonable maximum (200)".to_string());
        }
        if sell_leverage > 200.0 {
            return Err("sell_leverage exceeds reasonable maximum (200)".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_symmetric() {
        let request = LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "15");
        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.buy_leverage, "15");
        assert_eq!(request.sell_leverage, "15");
    }

    #[test]
    fn test_new_asymmetric() {
        let request = LeverageRequest::new(Category::Linear, "BTCUSDT", "20", "10");
        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.buy_leverage, "20");
        assert_eq!(request.sell_leverage, "10");
    }

    #[test]
    fn test_validation() {
        let valid_request = LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "10");
        assert!(valid_request.validate().is_ok());

        let invalid_request = LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "0.5");
        assert!(invalid_request.validate().is_err());

        let too_high_request = LeverageRequest::new_symmetric(Category::Linear, "BTCUSDT", "300");
        assert!(too_high_request.validate().is_err());
    }

    #[test]
    fn test_default() {
        let request = LeverageRequest::default();
        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.buy_leverage, "10");
        assert_eq!(request.sell_leverage, "10");
    }
}
