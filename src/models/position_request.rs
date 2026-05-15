use crate::prelude::*;

/// Parameters for requesting position information.
///
/// Used to construct a request to the `/v5/position/list` endpoint to retrieve current position details. Bots use this to monitor open positions, calculate unrealized P&L, and manage risk in perpetual futures.
#[derive(Clone, Default)]
pub struct PositionRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to fetch positions for the correct contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Optionally filters positions by symbol. If unset, all positions in the category are returned. Bots should specify this for targeted position monitoring.
    pub symbol: Option<Cow<'a, str>>,

    /// The base coin (e.g., "BTC").
    ///
    /// Optionally filters positions by the base asset. Useful for bots managing multiple pairs of the same asset.
    pub base_coin: Option<Cow<'a, str>>,

    /// The settlement coin (e.g., "USDT").
    ///
    /// Optionally filters positions by the settlement currency. For `Linear` perpetuals, this is typically "USDT". Bots can use this to focus on specific margin types.
    pub settle_coin: Option<Cow<'a, str>>,

    /// The maximum number of position records to return.
    ///
    /// Controls the number of position records returned. Bots should set a reasonable limit to balance data completeness with performance.
    pub limit: Option<usize>,
}

impl<'a> PositionRequest<'a> {
    /// Creates a default Position request.
    ///
    /// Returns a request with `category` set to `Linear` and all other fields unset. Suitable for broad queries but should be customized for specific position monitoring needs.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::new(Category::Linear, None, None, None, None)
    }
    /// Constructs a new Position request with specified parameters.
    ///
    /// Allows full customization. Bots should use this to specify the exact symbol, category, and filters to align with their position management requirements.
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        settle_coin: Option<&'a str>,
        limit: Option<usize>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            settle_coin: settle_coin.map(Cow::Borrowed),
            limit,
        }
    }
}

impl<'a> MovePositionRequest<'a> {
    /// Constructs a new MovePosition request with specified parameters.
    ///
    /// Allows customization of the position transfer request. Bots should use this to specify the source and destination UIDs and the list of positions to move.
    pub fn new(from_uid: u64, to_uid: u64, list: Vec<PositionItem<'a>>) -> Self {
        Self {
            from_uid,
            to_uid,
            list,
        }
    }
    /// Creates a default MovePosition request.
    ///
    /// Returns a request with `from_uid` and `to_uid` set to `0` and an empty position list. Suitable for testing but should be customized with valid UIDs and positions for production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> MovePositionRequest<'a> {
        MovePositionRequest::new(0, 0, vec![])
    }
}
