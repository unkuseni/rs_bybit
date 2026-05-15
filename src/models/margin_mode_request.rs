use crate::prelude::*;

/// Parameters for setting margin mode for an account or symbol.
///
/// Used to construct a request to the `/v5/account/set-margin-mode` endpoint to set the margin mode (cross or isolated) for an account or specific symbol. Bots use this to control margin allocation strategies across positions.
#[derive(Clone, Default)]
pub struct MarginModeRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type.
    pub category: Category,

    /// The margin mode (0 for cross margin, 1 for isolated margin).
    ///
    /// Specifies whether to use cross margin (shared across positions) or isolated margin (specific to each position). Bots use this to optimize margin usage.
    pub mode: i8,

    /// The trading pair symbol (e.g., "BTCUSDT") (optional).
    ///
    /// If specified, sets the margin mode for a specific symbol. If unset, applies to the entire account. Bots should specify this for targeted margin mode changes.
    pub symbol: Option<Cow<'a, str>>,

    /// The coin (e.g., "USDT") (optional).
    ///
    /// Optionally specifies the settlement currency. For `Linear` perpetuals, this is typically "USDT". Bots can use this to filter margin mode settings by currency.
    pub coin: Option<Cow<'a, str>>,
}

impl<'a> MarginModeRequest<'a> {
    /// Constructs a new MarginMode request with specified parameters.
    ///
    /// Allows customization of the margin mode request. Bots should use this to specify the category, mode, symbol, and coin as needed.
    pub fn new(
        category: Category,
        mode: i8,
        symbol: Option<&'a str>,
        coin: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            mode,
            symbol: symbol.map(Cow::Borrowed),
            coin: coin.map(Cow::Borrowed),
        }
    }
    /// Creates a default MarginMode request.
    ///
    /// Returns a request with `category` set to `Linear`, `mode` set to `1` (isolated margin), and no symbol or coin specified. Suitable for testing but should be customized for production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> MarginModeRequest<'a> {
        MarginModeRequest::new(Category::Linear, 1, None, None)
    }
}
