use crate::prelude::*;

/// Valid kline intervals supported by Bybit API.
///
/// This enum provides type-safe interval specification for kline requests.
/// Each variant corresponds to a valid interval string accepted by the Bybit API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interval {
    /// 1 minute
    M1,
    /// 3 minutes
    M3,
    /// 5 minutes
    M5,
    /// 15 minutes
    M15,
    /// 30 minutes
    M30,
    /// 1 hour
    H1,
    /// 2 hours
    H2,
    /// 4 hours
    H4,
    /// 6 hours
    H6,
    /// 12 hours
    H12,
    /// 1 day
    D1,
    /// 1 week
    W1,
    /// 1 month
    M,
}

impl Default for Interval {
    /// Returns the default interval (1 hour).
    fn default() -> Self {
        Interval::H1
    }
}

impl Interval {
    /// Returns the string representation of the interval as expected by the Bybit API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Interval::M1 => "1",
            Interval::M3 => "3",
            Interval::M5 => "5",
            Interval::M15 => "15",
            Interval::M30 => "30",
            Interval::H1 => "60",
            Interval::H2 => "120",
            Interval::H4 => "240",
            Interval::H6 => "360",
            Interval::H12 => "720",
            Interval::D1 => "D",
            Interval::W1 => "W",
            Interval::M => "M",
        }
    }

    /// Parses a string into an Interval enum.
    ///
    /// Returns `Some(Interval)` if the string is a valid interval, `None` otherwise.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" | "1m" => Some(Interval::M1),
            "3" | "3m" => Some(Interval::M3),
            "5" | "5m" => Some(Interval::M5),
            "15" | "15m" => Some(Interval::M15),
            "30" | "30m" => Some(Interval::M30),
            "60" | "1h" => Some(Interval::H1),
            "120" | "2h" => Some(Interval::H2),
            "240" | "4h" => Some(Interval::H4),
            "360" | "6h" => Some(Interval::H6),
            "720" | "12h" => Some(Interval::H12),
            "D" | "1d" => Some(Interval::D1),
            "W" | "1w" => Some(Interval::W1),
            "M" | "1M" => Some(Interval::M),
            _ => None,
        }
    }

    /// Returns the duration of the interval in seconds.
    pub fn duration_seconds(&self) -> u64 {
        match self {
            Interval::M1 => 60,
            Interval::M3 => 180,
            Interval::M5 => 300,
            Interval::M15 => 900,
            Interval::M30 => 1800,
            Interval::H1 => 3600,
            Interval::H2 => 7200,
            Interval::H4 => 14400,
            Interval::H6 => 21600,
            Interval::H12 => 43200,
            Interval::D1 => 86400,
            Interval::W1 => 604800,
            Interval::M => 2592000, // Approximate 30 days
        }
    }

    /// Returns the duration of the interval in milliseconds.
    pub fn duration_millis(&self) -> u64 {
        self.duration_seconds() * 1000
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Interval> for String {
    fn from(interval: Interval) -> Self {
        interval.as_str().to_string()
    }
}

impl<'a> From<&'a Interval> for Cow<'a, str> {
    fn from(interval: &'a Interval) -> Self {
        Cow::Borrowed(interval.as_str())
    }
}

/// Parameters for requesting Kline (candlestick) data.
///
/// Kline data represents price movements over fixed time intervals (e.g., 1-minute, 1-hour) and is used for technical analysis in trading. This struct defines the parameters for querying Kline data via Bybit's `/v5/market/kline` endpoint. Perpetual futures on Bybit, unlike traditional futures, have no expiry date and are funded via a funding rate mechanism, making Kline data critical for analyzing price trends in these instruments.
///
/// # Examples
///
/// ```rust
/// use bybit::prelude::*;
///
/// // Create a request using the builder pattern
/// let request = KlineRequest::builder()
///     .category(Category::Linear)
///     .symbol("BTCUSDT")
///     .interval(Interval::H1)
///     .limit(100)
///     .build();
///
/// // Or create a simple request
/// let simple_request = KlineRequest::simple("BTCUSDT", Interval::D1);
/// ```
#[derive(Clone, Default)]
pub struct KlineRequest<'a> {
    /// The product category (e.g., Spot, Linear, Inverse, Option).
    ///
    /// Specifies the type of instrument for the Kline data. For perpetual futures, `Linear` (USDT-margined) or `Inverse` (coin-margined) are most relevant. Bots should set this to match the trading pair (e.g., `Linear` for `BTCUSDT`). If unset, the API may return an error or default to an unexpected category.
    pub category: Option<Category>,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the specific perpetual futures contract or other instrument. For perpetuals, this is typically a USDT pair (e.g., `BTCUSDT`) for linear contracts or a coin pair (e.g., `BTCUSD`) for inverse contracts. Bots must ensure the symbol is valid for the chosen `category` to avoid errors.
    pub symbol: Cow<'a, str>,

    /// The time interval for each candlestick.
    ///
    /// Specifies the granularity of the Kline data. The choice depends on the trading strategy:
    /// - Short-term bots may use smaller intervals (e.g., `Interval::M1` for 1 minute)
    /// - Long-term strategies may use larger intervals (e.g., `Interval::D1` for 1 day)
    pub interval: Interval,

    /// The start time for the Kline data (Unix timestamp in milliseconds).
    ///
    /// Defines the beginning of the time range for the Kline data. For perpetual futures, this is useful for fetching historical data to backtest trading strategies. If unset, the API may return data from the most recent period, which may not suit historical analysis needs.
    pub start: Option<Cow<'a, str>>,

    /// The end time for the Kline data (Unix timestamp in milliseconds).
    ///
    /// Defines the end of the time range for the Kline data. Bots should set this to limit the data to a specific period, especially for performance optimization when processing large datasets. If unset, the API typically returns data up to the current time.
    pub end: Option<Cow<'a, str>>,

    /// The maximum number of Kline records to return (1-1000, default: 200).
    ///
    /// Controls the number of candlesticks returned in the response. For trading bots, setting a lower limit can reduce latency and memory usage, especially for high-frequency strategies. However, for comprehensive analysis, bots may need to paginate through multiple requests to fetch all desired data.
    pub limit: Option<u64>,
}

impl<'a> KlineRequest<'a> {
    /// Creates a default Kline request with preset values.
    ///
    /// Returns a `KlineRequest` with `symbol` set to `"BTCUSDT"`, `interval` set to `Interval::H1` (1 hour),
    /// and other fields as defaults. Useful for quick testing or prototyping trading bots but should be
    /// customized for production to match specific trading pairs and intervals.
    pub fn default() -> KlineRequest<'a> {
        KlineRequest {
            category: None,
            symbol: Cow::Borrowed("BTCUSDT"),
            interval: Interval::H1,
            start: None,
            end: None,
            limit: None,
        }
    }

    /// Constructs a new Kline request with specified parameters.
    ///
    /// Allows full customization of the Kline request. Trading bots should use this to specify exact
    /// parameters for their strategy, ensuring the `symbol`, `interval`, and `category` align with
    /// the perpetual futures contract being traded.
    ///
    /// # Arguments
    ///
    /// * `category` - The product category (Spot, Linear, Inverse, Option)
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    /// * `interval` - The time interval for each candlestick
    /// * `start` - Optional start time (Unix timestamp in milliseconds as string)
    /// * `end` - Optional end time (Unix timestamp in milliseconds as string)
    /// * `limit` - Optional maximum number of records (1-1000)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bybit::prelude::*;
    ///
    /// let request = KlineRequest::new(
    ///     Some(Category::Linear),
    ///     "ETHUSDT",
    ///     Interval::M15,
    ///     Some("1670601600000"),
    ///     Some("1670608800000"),
    ///     Some(100),
    /// );
    /// ```
    pub fn new(
        category: Option<Category>,
        symbol: &'a str,
        interval: Interval,
        start: Option<&'a str>,
        end: Option<&'a str>,
        limit: Option<u64>,
    ) -> KlineRequest<'a> {
        KlineRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            interval,
            start: start.map(Cow::Borrowed),
            end: end.map(Cow::Borrowed),
            limit,
        }
    }

    /// Creates a simple Kline request with just symbol and interval.
    ///
    /// This is a convenience method for common use cases where only the symbol
    /// and interval are specified, with defaults for other parameters.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    /// * `interval` - The time interval for each candlestick
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bybit::prelude::*;
    ///
    /// let request = KlineRequest::simple("BTCUSDT", Interval::D1);
    /// ```
    pub fn simple(symbol: &'a str, interval: Interval) -> Self {
        KlineRequest {
            category: None,
            symbol: Cow::Borrowed(symbol),
            interval,
            start: None,
            end: None,
            limit: None,
        }
    }

    /// Creates a builder for constructing KlineRequest with a fluent interface.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bybit::prelude::*;
    ///
    /// let request = KlineRequest::builder()
    ///     .category(Category::Linear)
    ///     .symbol("BTCUSDT")
    ///     .interval(Interval::H1)
    ///     .start("1670601600000")
    ///     .end("1670608800000")
    ///     .limit(100)
    ///     .build();
    /// ```
    pub fn builder() -> KlineRequestBuilder<'a> {
        KlineRequestBuilder::default()
    }

    /// Validates the request parameters.
    ///
    /// Checks that all parameters are within valid ranges and combinations.
    /// Returns `Ok(())` if valid, or an error message if invalid.
    ///
    /// # Errors
    ///
    /// Returns an error string if:
    /// - Limit is outside the range 1-1000
    /// - Start time is after end time (if both are specified)
    /// - Category is invalid for the specific kline type (checked by API methods)
    pub fn validate(&self) -> Result<(), String> {
        // Validate limit range
        if let Some(limit) = self.limit {
            if limit < 1 || limit > 1000 {
                return Err(format!("Limit must be between 1 and 1000, got {}", limit));
            }
        }

        // Validate start <= end if both are specified
        if let (Some(start_str), Some(end_str)) = (&self.start, &self.end) {
            if let (Ok(start), Ok(end)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) {
                if start > end {
                    return Err(format!(
                        "Start time ({}) cannot be after end time ({})",
                        start, end
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Builder for constructing `KlineRequest` with a fluent interface.
///
/// This builder provides a convenient way to construct `KlineRequest` objects
/// with method chaining. It validates parameters at build time to ensure
/// the request is valid before sending it to the API.
///
/// # Examples
///
/// ```rust
/// use bybit::prelude::*;
///
/// let request = KlineRequest::builder()
///     .category(Category::Linear)
///     .symbol("BTCUSDT")
///     .interval(Interval::H1)
///     .start("1670601600000")
///     .end("1670608800000")
///     .limit(100)
///     .build();
/// ```
#[derive(Default)]
pub struct KlineRequestBuilder<'a> {
    category: Option<Category>,
    symbol: Option<Cow<'a, str>>,
    interval: Option<Interval>,
    start: Option<Cow<'a, str>>,
    end: Option<Cow<'a, str>>,
    limit: Option<u64>,
}

impl<'a> KlineRequestBuilder<'a> {
    /// Sets the product category.
    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    /// Sets the trading pair symbol.
    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(Cow::Borrowed(symbol));
        self
    }

    /// Sets the time interval.
    pub fn interval(mut self, interval: Interval) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Sets the start time (Unix timestamp in milliseconds as string).
    pub fn start(mut self, start: &'a str) -> Self {
        self.start = Some(Cow::Borrowed(start));
        self
    }

    /// Sets the end time (Unix timestamp in milliseconds as string).
    pub fn end(mut self, end: &'a str) -> Self {
        self.end = Some(Cow::Borrowed(end));
        self
    }

    /// Sets the maximum number of records to return (1-1000).
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Builds the `KlineRequest` and validates the parameters.
    ///
    /// # Returns
    ///
    /// Returns `Ok(KlineRequest)` if all required parameters are set and valid,
    /// or an error string if validation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Symbol is not set
    /// - Interval is not set
    /// - Limit is outside valid range (1-1000)
    /// - Start time is after end time
    pub fn build(self) -> Result<KlineRequest<'a>, String> {
        let symbol = self
            .symbol
            .ok_or_else(|| "Symbol is required".to_string())?;
        let interval = self
            .interval
            .ok_or_else(|| "Interval is required".to_string())?;

        let request = KlineRequest {
            category: self.category,
            symbol,
            interval,
            start: self.start,
            end: self.end,
            limit: self.limit,
        };

        request.validate()?;
        Ok(request)
    }
}
