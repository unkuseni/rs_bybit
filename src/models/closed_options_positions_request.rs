use crate::prelude::*;

/// Parameters for requesting closed options positions.
///
/// Used to construct a request to the `/v5/position/get-closed-positions` endpoint to retrieve closed options positions.
/// Bots use this to analyze historical options trading performance, track P&L, and audit trading activity.
///
/// # Bybit API Reference
/// According to the Bybit V5 API documentation:
/// - Only supports querying closed options positions in the last 6 months.
/// - Sorted by `closeTime` in descending order.
/// - Fee and price are displayed with trailing zeroes up to 8 decimal places.
///
/// # Usage Example
/// ```rust
/// // Query closed options positions for a specific symbol
/// let request = ClosedOptionsPositionsRequest::new(
///     Category::Option,
///     Some("BTC-12JUN25-104019-C-USDT"),
///     None,
///     None,
///     Some(50),
///     None,
/// );
///
/// // Query all closed options positions from the last day
/// let request = ClosedOptionsPositionsRequest::new(
///     Category::Option,
///     None,
///     Some(1749730000000), // startTime in milliseconds
///     None,
///     None,
///     None,
/// );
/// ```
#[derive(Clone, Default)]
pub struct ClosedOptionsPositionsRequest<'a> {
    /// The product category (must be `option`).
    ///
    /// Specifies the instrument type. For this endpoint, must be `Category::Option`.
    pub category: Category,

    /// The options symbol name (e.g., "BTC-12JUN25-104019-C-USDT") (optional).
    ///
    /// Filters closed positions by symbol. If unset, all closed options positions are returned.
    /// Bots should specify this for targeted analysis of specific options contracts.
    pub symbol: Option<Cow<'a, str>>,

    /// The start timestamp in milliseconds (optional).
    ///
    /// The beginning of the time range for querying closed positions.
    /// According to Bybit API:
    /// - If neither `start_time` nor `end_time` are provided, returns data from the last 1 day
    /// - If only `start_time` is provided, returns range between `start_time` and `start_time + 1 day`
    /// - If only `end_time` is provided, returns range between `end_time - 1 day` and `end_time`
    /// - If both are provided, the rule is `end_time - start_time <= 7 days`
    pub start_time: Option<u64>,

    /// The end timestamp in milliseconds (optional).
    ///
    /// The end of the time range for querying closed positions.
    pub end_time: Option<u64>,

    /// The maximum number of records to return (optional).
    ///
    /// Limit for data size per page. Valid range: [`1`, `100`]. Default: `50`.
    /// Bots should use pagination with the `cursor` field for large result sets.
    pub limit: Option<usize>,

    /// The cursor for pagination (optional).
    ///
    /// Use the `nextPageCursor` token from the response to retrieve the next page of results.
    /// Bots use this to efficiently paginate through large sets of closed positions.
    pub cursor: Option<Cow<'a, str>>,
}

impl<'a> ClosedOptionsPositionsRequest<'a> {
    /// Constructs a new ClosedOptionsPositions request with specified parameters.
    ///
    /// Allows full customization of the closed options positions query.
    /// Bots should use this to specify time ranges, symbols, and pagination parameters.
    ///
    /// # Arguments
    /// * `category` - The product category (must be `Category::Option`)
    /// * `symbol` - The options symbol name (optional)
    /// * `start_time` - The start timestamp in milliseconds (optional)
    /// * `end_time` - The end timestamp in milliseconds (optional)
    /// * `limit` - The maximum number of records to return (optional, range: 1-100)
    /// * `cursor` - The cursor for pagination (optional)
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<usize>,
        cursor: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            start_time,
            end_time,
            limit,
            cursor: cursor.map(Cow::Borrowed),
        }
    }

    /// Creates a default ClosedOptionsPositions request.
    ///
    /// Returns a request with `category` set to `Option`, no symbol filter,
    /// no time range (returns last 1 day by default), limit set to `50`, and no cursor.
    /// Suitable for testing but should be customized for production analysis.
    pub fn default() -> ClosedOptionsPositionsRequest<'a> {
        ClosedOptionsPositionsRequest::new(Category::Option, None, None, None, Some(50), None)
    }

    /// Validates the request parameters according to Bybit API constraints.
    ///
    /// Bots should call this method before sending the request to ensure compliance with API limits.
    ///
    /// # Returns
    /// * `Ok(())` if validation passes
    /// * `Err(String)` with error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        // Category must be Option
        if !matches!(self.category, Category::Option) {
            return Err("Category must be 'option' for closed options positions".to_string());
        }

        // Validate limit range
        if let Some(limit) = self.limit {
            if limit < 1 || limit > 100 {
                return Err("Limit must be between 1 and 100".to_string());
            }
        }

        // Validate time range if both start_time and end_time are provided
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            if start >= end {
                return Err("start_time must be less than end_time".to_string());
            }

            // Check if time range exceeds 7 days (604800000 milliseconds)
            if end - start > 7 * 24 * 60 * 60 * 1000 {
                return Err("Time range cannot exceed 7 days".to_string());
            }
        }

        Ok(())
    }

    /// Sets the symbol filter for the request.
    ///
    /// Convenience method for updating the symbol filter.
    ///
    /// # Arguments
    /// * `symbol` - The options symbol name
    pub fn with_symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(Cow::Borrowed(symbol));
        self
    }

    /// Sets the time range for the request.
    ///
    /// Convenience method for updating both start and end times.
    ///
    /// # Arguments
    /// * `start_time` - The start timestamp in milliseconds
    /// * `end_time` - The end timestamp in milliseconds
    pub fn with_time_range(mut self, start_time: u64, end_time: u64) -> Self {
        self.start_time = Some(start_time);
        self.end_time = Some(end_time);
        self
    }

    /// Sets the limit for the request.
    ///
    /// Convenience method for updating the result limit.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of records to return (1-100)
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the cursor for pagination.
    ///
    /// Convenience method for updating the pagination cursor.
    ///
    /// # Arguments
    /// * `cursor` - The cursor string from previous response
    pub fn with_cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(Cow::Borrowed(cursor));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let request = ClosedOptionsPositionsRequest::new(
            Category::Option,
            Some("BTC-12JUN25-104019-C-USDT"),
            Some(1749730000000),
            Some(1749736000000),
            Some(25),
            Some("cursor_token"),
        );

        assert_eq!(request.category, Category::Option);
        assert_eq!(request.symbol.unwrap(), "BTC-12JUN25-104019-C-USDT");
        assert_eq!(request.start_time.unwrap(), 1749730000000);
        assert_eq!(request.end_time.unwrap(), 1749736000000);
        assert_eq!(request.limit.unwrap(), 25);
        assert_eq!(request.cursor.unwrap(), "cursor_token");
    }

    #[test]
    fn test_default() {
        let request = ClosedOptionsPositionsRequest::default();
        assert_eq!(request.category, Category::Option);
        assert!(request.symbol.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert_eq!(request.limit.unwrap(), 50);
        assert!(request.cursor.is_none());
    }

    #[test]
    fn test_validation() {
        // Valid request
        let valid_request =
            ClosedOptionsPositionsRequest::new(Category::Option, None, None, None, Some(50), None);
        assert!(valid_request.validate().is_ok());

        // Invalid category
        let invalid_category =
            ClosedOptionsPositionsRequest::new(Category::Linear, None, None, None, Some(50), None);
        assert!(invalid_category.validate().is_err());

        // Invalid limit (too low)
        let invalid_limit_low =
            ClosedOptionsPositionsRequest::new(Category::Option, None, None, None, Some(0), None);
        assert!(invalid_limit_low.validate().is_err());

        // Invalid limit (too high)
        let invalid_limit_high =
            ClosedOptionsPositionsRequest::new(Category::Option, None, None, None, Some(101), None);
        assert!(invalid_limit_high.validate().is_err());

        // Invalid time range (start >= end)
        let invalid_time_range = ClosedOptionsPositionsRequest::new(
            Category::Option,
            None,
            Some(1749736000000),
            Some(1749730000000),
            Some(50),
            None,
        );
        assert!(invalid_time_range.validate().is_err());

        // Invalid time range (exceeds 7 days)
        let invalid_time_range_long = ClosedOptionsPositionsRequest::new(
            Category::Option,
            None,
            Some(1749730000000),
            Some(1749730000000 + 8 * 24 * 60 * 60 * 1000), // 8 days later
            Some(50),
            None,
        );
        assert!(invalid_time_range_long.validate().is_err());
    }

    #[test]
    fn test_builder_methods() {
        let request = ClosedOptionsPositionsRequest::default()
            .with_symbol("BTC-12JUN25-104019-C-USDT")
            .with_time_range(1749730000000, 1749736000000)
            .with_limit(25)
            .with_cursor("cursor_token");

        assert_eq!(request.category, Category::Option);
        assert_eq!(request.symbol.unwrap(), "BTC-12JUN25-104019-C-USDT");
        assert_eq!(request.start_time.unwrap(), 1749730000000);
        assert_eq!(request.end_time.unwrap(), 1749736000000);
        assert_eq!(request.limit.unwrap(), 25);
        assert_eq!(request.cursor.unwrap(), "cursor_token");
    }
}
