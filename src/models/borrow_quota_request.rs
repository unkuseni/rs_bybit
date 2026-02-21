use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::models::category::Category;
use crate::models::side::Side;

/// Represents a request to check borrow quota for spot trading on Bybit.
///
/// This struct is used to query the available balance for Spot trading and Margin trading,
/// including both actual balances and maximum borrowable amounts. Bots use this to determine
/// available trading capacity before placing orders, ensuring sufficient funds and managing
/// position sizing in spot and margin trading scenarios.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BorrowQuotaRequest<'a> {
    /// Product type, must be "spot".
    ///
    /// This endpoint only supports the spot category. Bots must set this to `Category::Spot`
    /// to ensure the request is valid. Using other categories will result in API errors.
    pub category: Category,

    /// Symbol name, e.g., "BTCUSDT".
    ///
    /// Identifies the trading pair for which to check borrow quota. The symbol must be
    /// uppercase and valid on Bybit's platform. Bots should verify symbol availability
    /// before making the request to avoid errors.
    pub symbol: Cow<'a, str>,

    /// Transaction side: "Buy" or "Sell".
    ///
    /// Determines whether to check quota for buying or selling the base asset.
    /// Quota values differ based on side due to different margin requirements
    /// and available balances for each direction. Bots must set this correctly
    /// based on their intended trade direction.
    pub side: Side,
}

impl<'a> BorrowQuotaRequest<'a> {
    /// Constructs a new BorrowQuotaRequest with specified parameters.
    ///
    /// Creates a request to check borrow quota for a specific symbol and side.
    /// Bots should use this to determine available trading capacity before
    /// placing orders in spot or margin trading.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    /// * `side` - The transaction side (Buy or Sell)
    ///
    /// # Returns
    ///
    /// A new `BorrowQuotaRequest` instance with category set to `Category::Spot`.
    pub fn new(symbol: &'a str, side: Side) -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed(symbol),
            side,
        }
    }

    /// Creates a default BorrowQuotaRequest for BTCUSDT Buy.
    ///
    /// Returns a request with BTCUSDT symbol and Buy side as defaults.
    /// Suitable for testing or as a template that can be modified.
    /// Bots should customize the symbol and side for actual trading scenarios.
    pub fn default() -> Self {
        Self {
            category: Category::Spot,
            symbol: Cow::Borrowed("BTCUSDT"),
            side: Side::Buy,
        }
    }
}
