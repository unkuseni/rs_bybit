use crate::prelude::*;

/// Parameters for requesting transaction log data.
///
/// Used to construct a request to the `/v5/account/transaction-log` endpoint to retrieve historical transaction records, such as trades, fees, and funding. Bots use this to audit trading activity, calculate costs, and analyze performance in perpetual futures trading.
#[derive(Clone, Default)]
pub struct TransactionLogRequest<'a> {
    /// The account type to filter logs (e.g., "UNIFIED", "SPOT") (optional).
    ///
    /// Optionally filters transactions by account type. If unset, data for all account types is returned. Bots should specify this to focus on specific account activities.
    pub account_type: Option<Cow<'a, str>>,

    /// The product category (e.g., Linear, Inverse) (optional).
    ///
    /// Optionally filters transactions by instrument type. If unset, data for all categories is returned. Bots should specify this to analyze specific contract types.
    pub category: Option<Category>,

    /// The currency to filter logs (e.g., "USDT") (optional).
    ///
    /// Optionally filters transactions by settlement currency. If unset, data for all currencies is returned. Bots should specify this for targeted financial analysis.
    pub currency: Option<Cow<'a, str>>,

    /// The base coin to filter logs (e.g., "BTC") (optional).
    ///
    /// Optionally filters transactions by base asset. If unset, data for all base coins is returned. Bots should specify this to focus on specific trading pairs.
    pub base_coin: Option<Cow<'a, str>>,

    /// The transaction type to filter logs (e.g., "TRADE", "FUNDING") (optional).
    ///
    /// Optionally filters transactions by type, such as trades or funding fees. If unset, all transaction types are returned. Bots should specify this to analyze specific activities.
    pub log_type: Option<Cow<'a, str>>,

    /// The start time for the transaction log (Unix timestamp in milliseconds) (optional).
    ///
    /// Defines the beginning of the time range. Bots should set this to focus on a specific historical period, such as a trading session.
    pub start_time: Option<u64>,

    /// The end time for the transaction log (Unix timestamp in milliseconds) (optional).
    ///
    /// Defines the end of the time range. Bots should set this to limit data to a specific period, optimizing performance.
    pub end_time: Option<u64>,

    /// The maximum number of transaction records to return (optional).
    ///
    /// Controls the number of records returned (e.g., max 50). Bots should set a reasonable limit to balance data completeness with performance.
    pub limit: Option<u32>,
}

impl<'a> TransactionLogRequest<'a> {
    /// Constructs a new TransactionLog request with specified parameters.
    ///
    /// Allows customization of the transaction log request. Bots should use this to specify the exact filters and time range to align with their analysis needs.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_type: Option<&'a str>,
        category: Option<Category>,
        currency: Option<&'a str>,
        base_coin: Option<&'a str>,
        log_type: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Self {
        Self {
            account_type: account_type.map(Cow::Borrowed),
            category,
            currency: currency.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            log_type: log_type.map(Cow::Borrowed),
            start_time,
            end_time,
            limit,
        }
    }
    /// Creates a default TransactionLog request.
    ///
    /// Returns a request with all fields unset. Suitable for broad queries but should be customized for specific analysis needs in production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::new(None, None, None, None, None, None, None, None)
    }
}
