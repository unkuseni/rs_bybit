use crate::prelude::*;

/// Represents a risk limit configuration for a trading symbol.
/// Defines leverage, margin, and position size limits for perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimit {
    /// Unique identifier for the risk limit tier.
    /// Used by Bybit to distinguish different risk levels. Bots can use this to track specific
    /// configurations.
    pub id: u64,

    /// The trading symbol (e.g., "BTCUSDT").
    /// Specifies the market to which the risk limit applies. Bots must match this with their
    /// trading pairs.
    pub symbol: String,

    /// The maximum position size allowed (in base currency or USD).
    /// In perpetual futures, this limits exposure to prevent excessive risk. Bots use this to
    /// cap order sizes and avoid rejections.
    #[serde(with = "string_to_float")]
    pub risk_limit_value: f64,

    /// The maintenance margin rate (e.g., 0.005 for 0.5%).
    /// The minimum margin required to keep a position open. If the margin falls below this,

    /// liquidation occurs at the bust price (bankruptcy price). Bots monitor this to manage
    /// liquidation risks.
    #[serde(with = "string_to_float")]
    pub maintenance_margin: f64,

    /// The initial margin rate (e.g., 0.01 for 1%).
    /// The margin required to open a position. Lower rates allow higher leverage, but increase
    /// liquidation risk. Bots use this to calculate capital requirements.
    #[serde(with = "string_to_float")]
    pub initial_margin: f64,

    /// Indicates if this is the lowest risk tier (1 for true, 0 for false).
    /// Lower risk tiers have stricter limits but safer margin requirements. Bots may prefer these
    /// for conservative strategies.
    pub is_lowest_risk: u8,

    /// The maximum leverage allowed (e.g., "100" for 100x).
    /// Leverage amplifies gains and losses in perpetual futures. Bots must ensure orders comply
    /// with this limit to avoid rejections.
    #[serde(with = "string_to_float")]
    pub max_leverage: f64,
}

impl<'a> SetRiskLimit<'a> {
    /// Constructs a new SetRiskLimit request with specified parameters.
    ///
    /// Allows customization of the risk limit request. Bots should use this to specify the category, symbol, risk ID, and position index as needed.
    pub fn new(
        category: Category,
        symbol: &'a str,
        risk_id: i8,
        position_idx: Option<i32>,
    ) -> Self {
        Self {
            category,
            symbol: Cow::Borrowed(symbol),
            risk_id,
            position_idx,
        }
    }
    /// Creates a default SetRiskLimit request.
    ///
    /// Returns a request with `category` set to `Linear`, `symbol` set to `"BTCUSDT"`, `risk_id` set to `1`, and no position index. Suitable for testing but should be customized for production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> SetRiskLimit<'a> {
        SetRiskLimit::new(Category::Linear, "BTCUSDT", 1, None)
    }
}
