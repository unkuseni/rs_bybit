use crate::prelude::*;

/// Parameters for requesting new delivery price data for options contracts.
///
/// This struct defines the parameters for querying the new delivery price via the
/// `/v5/market/new-delivery-price` endpoint. This endpoint returns historical option
/// delivery prices, with the most recent 50 records returned in reverse order of
/// "deliveryTime" by default.
///
/// # Important Notes
/// - This endpoint is specifically for options contracts only (`category` must be `option`)
/// - It is recommended to query this endpoint 1 minute after settlement is completed,
///   because the data returned by this endpoint may be delayed by 1 minute.
#[derive(Clone, Default)]
pub struct NewDeliveryPriceRequest<'a> {
    /// The product category (must be `option`).
    ///
    /// This endpoint only supports options contracts. The value must be `Category::Option`.
    /// This parameter is required.
    pub category: Category,

    /// The base coin (e.g., "BTC", "ETH").
    ///
    /// Specifies the underlying asset for the options contracts.
    /// This parameter is required and must be in uppercase.
    pub base_coin: Cow<'a, str>,

    /// The settle coin (e.g., "USDT", "USDC").
    ///
    /// Specifies the settlement currency for the options contracts.
    /// This parameter is optional and defaults to "USDT" if not specified.
    pub settle_coin: Option<Cow<'a, str>>,
}

impl<'a> NewDeliveryPriceRequest<'a> {
    /// Creates a default NewDeliveryPrice request for BTC options.
    ///
    /// Returns a request with `category` set to `Option`, `base_coin` set to `"BTC"`,
    /// and no settle coin (defaults to USDT).
    /// Suitable for testing but should be customized for production.
    pub fn default() -> NewDeliveryPriceRequest<'a> {
        NewDeliveryPriceRequest::new(Category::Option, "BTC", None)
    }

    /// Constructs a new NewDeliveryPrice request with specified parameters.
    ///
    /// Allows customization of the request parameters. Bots should use this to specify
    /// the exact base coin and optional settle coin for options delivery price queries.
    ///
    /// # Arguments
    ///
    /// * `category` - The product category (must be `Category::Option`)
    /// * `base_coin` - The base coin (e.g., "BTC", "ETH")
    /// * `settle_coin` - Optional settle coin (e.g., "USDT", "USDC")
    ///
    /// # Panics
    ///
    /// Panics if `category` is not `Category::Option`.
    pub fn new(
        category: Category,
        base_coin: &'a str,
        settle_coin: Option<&'a str>,
    ) -> NewDeliveryPriceRequest<'a> {
        // Validate that category is Option
        if category != Category::Option {
            panic!("NewDeliveryPrice endpoint only supports options contracts (category must be 'option')");
        }

        NewDeliveryPriceRequest {
            category,
            base_coin: Cow::Borrowed(base_coin),
            settle_coin: settle_coin.map(Cow::Borrowed),
        }
    }

    /// Constructs a new NewDeliveryPrice request with specified parameters, returning a Result.
    ///
    /// Similar to `new`, but returns a Result instead of panicking on invalid parameters.
    /// This is the recommended method for production code.
    ///
    /// # Arguments
    ///
    /// * `category` - The product category (must be `Category::Option`)
    /// * `base_coin` - The base coin (e.g., "BTC", "ETH")
    /// * `settle_coin` - Optional settle coin (e.g., "USDT", "USDC")
    ///
    /// # Returns
    ///
    /// Returns `Ok(NewDeliveryPriceRequest)` if parameters are valid, or `Err(String)` with an error message.
    pub fn try_new(
        category: Category,
        base_coin: &'a str,
        settle_coin: Option<&'a str>,
    ) -> Result<NewDeliveryPriceRequest<'a>, String> {
        // Validate that category is Option
        if category != Category::Option {
            return Err(
                "NewDeliveryPrice endpoint only supports options contracts (category must be 'option')"
                    .to_string(),
            );
        }

        Ok(NewDeliveryPriceRequest {
            category,
            base_coin: Cow::Borrowed(base_coin),
            settle_coin: settle_coin.map(Cow::Borrowed),
        })
    }

    /// Creates a NewDeliveryPriceRequest for BTC options with USDT settlement.
    ///
    /// Convenience method for creating requests for BTC options.
    pub fn btc() -> Result<NewDeliveryPriceRequest<'a>, String> {
        Self::try_new(Category::Option, "BTC", None)
    }

    /// Creates a NewDeliveryPriceRequest for ETH options with USDT settlement.
    ///
    /// Convenience method for creating requests for ETH options.
    pub fn eth() -> Result<NewDeliveryPriceRequest<'a>, String> {
        Self::try_new(Category::Option, "ETH", None)
    }

    /// Creates a NewDeliveryPriceRequest for SOL options with USDT settlement.
    ///
    /// Convenience method for creating requests for SOL options.
    pub fn sol() -> Result<NewDeliveryPriceRequest<'a>, String> {
        Self::try_new(Category::Option, "SOL", None)
    }

    /// Creates a NewDeliveryPriceRequest with USDT settlement.
    ///
    /// Convenience method for creating requests with USDT settlement.
    pub fn usdt(base_coin: &'a str) -> Result<NewDeliveryPriceRequest<'a>, String> {
        Self::try_new(Category::Option, base_coin, Some("USDT"))
    }

    /// Creates a NewDeliveryPriceRequest with USDC settlement.
    ///
    /// Convenience method for creating requests with USDC settlement.
    pub fn usdc(base_coin: &'a str) -> Result<NewDeliveryPriceRequest<'a>, String> {
        Self::try_new(Category::Option, base_coin, Some("USDC"))
    }

    /// Sets the settle coin for the request.
    ///
    /// Returns a new request with the specified settle coin.
    pub fn with_settle_coin(mut self, settle_coin: &'a str) -> Self {
        self.settle_coin = Some(Cow::Borrowed(settle_coin));
        self
    }

    /// Removes the settle coin from the request.
    ///
    /// Returns a new request without a settle coin (will default to USDT).
    pub fn without_settle_coin(mut self) -> Self {
        self.settle_coin = None;
        self
    }
}
