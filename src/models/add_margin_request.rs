use crate::prelude::*;

/// Parameters for enabling or disabling auto-add margin for a position.
///
/// Used to construct a request to the `/v5/position/set-auto-add-margin` endpoint to enable or disable automatic margin addition for a specific position. Bots use this to control margin behavior, preventing unwanted margin calls or optimizing capital allocation in perpetual futures.
#[derive(Clone, Default)]
pub struct AddMarginRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type, such as `Linear` for USDT-margined perpetuals.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for which auto-add margin is being configured. Bots must specify a valid symbol.
    pub symbol: Cow<'a, str>,

    /// Whether to enable auto-add margin.
    ///
    /// If `true`, Bybit will automatically add margin to prevent liquidation when margin levels are low. If `false`, no automatic margin addition occurs, increasing liquidation risk. Bots should set this based on their risk tolerance and capital management strategy.
    pub auto_add: bool,

    /// The position index (optional, e.g., 0 for one-way mode, 1 or 2 for hedge mode).
    ///
    /// Specifies the position type. Bots should set this for hedge mode positions to target the correct side (e.g., long or short). If unset, applies to the default position.
    pub position_idx: Option<i32>,
}

impl<'a> AddMarginRequest<'a> {
    /// Constructs a new AddMargin request with specified parameters.
    ///
    /// Allows customization of the auto-add margin request. Bots should use this to specify the exact symbol, category, and auto-add setting to align with their margin management strategy.
    pub fn new(
        category: Category,
        symbol: &'a str,
        auto_add: bool,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            auto_add,
            position_idx,
        }
    }
    /// Creates a default AddMargin request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, `auto_add` set to `false`, and no position index. Suitable for testing but should be customized for production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> AddMarginRequest<'a> {
        AddMarginRequest::new(Category::Linear, "BTCUSDT", false, None)
    }
}
