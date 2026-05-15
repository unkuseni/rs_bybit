use crate::prelude::*;

/// Parameters for manually adding or reducing margin for a position.
///
/// Used to construct a request to the `/v5/position/add-margin` endpoint to manually adjust the margin allocated to a specific position. Bots use this to increase margin to avoid liquidation or reduce margin to free up capital in perpetual futures trading.
#[derive(Clone, Default)]
pub struct AddReduceMarginRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for which margin is being adjusted. Bots must specify a valid symbol.
    pub symbol: Cow<'a, str>,

    /// The margin amount to add (positive) or reduce (negative).
    ///
    /// A positive value adds margin to the position, reducing liquidation risk. A negative value reduces margin, freeing up capital but increasing risk. Bots should calculate this based on position size and margin requirements.
    pub margin: f64,

    /// The position index (optional, e.g., 0 for one-way mode, 1 or 2 for hedge mode).
    ///
    /// Specifies the position type. Bots should set this for hedge mode positions to target the correct side. If unset, applies to the default position.
    pub position_idx: Option<i32>,
}

impl<'a> AddReduceMarginRequest<'a> {
    /// Constructs a new AddReduceMargin request with specified parameters.
    ///
    /// Allows customization of the margin adjustment request. Bots should use this to specify the exact symbol, category, margin amount, and position index.
    pub fn new(
        category: Category,
        symbol: &'a str,
        margin: f64,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            margin,
            position_idx,
        }
    }
    /// Creates a default AddReduceMargin request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, `margin` set to `1.0`, and no position index. Suitable for testing but should be customized for production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> AddReduceMarginRequest<'a> {
        AddReduceMarginRequest::new(Category::Linear, "BTCUSDT", 1.0, None)
    }
}
