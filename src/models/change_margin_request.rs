use crate::prelude::*;

/// Parameters for changing margin settings for a position.
///
/// Used to construct a request to the `/v5/position/switch-margin` endpoint to switch between cross and isolated margin modes or adjust leverage. Bots use this to optimize margin usage and manage risk in perpetual futures.
#[derive(Default, Clone)]
pub struct ChangeMarginRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for which margin settings are being changed. Bots must specify a valid symbol.
    pub symbol: Cow<'a, str>,

    /// The trade mode (0 for cross margin, 1 for isolated margin).
    ///
    /// Specifies whether to use cross margin (shared across positions) or isolated margin (specific to this position). Bots use this to control margin allocation.
    pub trade_mode: i8,

    /// The leverage value (e.g., 10 for 10x).
    ///
    /// The desired leverage multiplier. Bots should ensure this complies with Bybit’s limits for the symbol and trade mode.
    pub leverage: i8,
}

impl<'a> ChangeMarginRequest<'a> {
    /// Constructs a new ChangeMargin request with specified parameters.
    ///
    /// Allows customization of the margin change request. Bots should use this to specify the exact symbol, category, trade mode, and leverage.
    pub fn new(category: Category, symbol: &'a str, trade_mode: i8, leverage: i8) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            trade_mode: match trade_mode {
                1 => 1,
                0 => 0,
                _ => 0,
            },
            leverage,
        }
    }
    /// Creates a default ChangeMargin request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, `trade_mode` set to `0` (cross margin), and `leverage` set to `10`. Suitable for testing but should be customized for production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ChangeMarginRequest<'a> {
        ChangeMarginRequest::new(Category::Linear, "BTCUSDT", 0, 10)
    }
}
