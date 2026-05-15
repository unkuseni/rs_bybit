use crate::prelude::*;

/// Represents a single position to be moved between accounts.
///
/// Part of the `MovePositionRequest`, this struct details the position to transfer, including its category, symbol, and trade parameters. Bots use this to specify which positions to move and at what price.
#[derive(Clone, Default, Serialize)]
pub struct PositionItem<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type of the position. Bots must set this to match the position’s contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the position. Bots must specify a valid symbol.
    pub symbol: Cow<'a, str>,

    /// The transfer price.
    ///
    /// The price at which the position is transferred, used for valuation between accounts. Bots should set this based on the current mark price or a mutually agreed value to ensure fair transfer.
    pub price: f64,

    /// The position side (Buy or Sell).
    ///
    /// Indicates whether the position is long (Buy) or short (Sell). Bots use this to ensure the correct position direction is transferred.
    pub side: Side,

    /// The quantity of the position to move.
    ///
    /// The amount of the base asset to transfer. Bots should ensure this does not exceed the available position size in the source account.
    pub qty: f64,
}

impl<'a> PositionItem<'a> {
    /// Constructs a new PositionItem with specified parameters.
    ///
    /// Allows customization of the position to move. Bots should use this to specify the category, symbol, price, side, and quantity of the position.
    pub fn new(category: Category, symbol: &'a str, price: f64, side: Side, qty: f64) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            price,
            side,
            qty,
        }
    }
    /// Creates a default PositionItem.
    ///
    /// Returns a position item with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, `price` and `qty` set to `0.0`, and `side` set to `Buy`. Suitable for testing but should be customized for production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> PositionItem<'a> {
        PositionItem::new(Category::Linear, "BTCUSDT", 0.0, Side::Buy, 0.0)
    }
}
