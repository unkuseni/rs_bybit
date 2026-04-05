/// Parameters for WebSocket subscription requests.
///
/// Used to construct a WebSocket subscription request to subscribe to real-time data streams, such as order book updates or trade events. Bots use this to configure WebSocket feeds for market monitoring and trading signals in perpetual futures trading.
#[derive(Clone, Debug, Default)]
pub struct Subscription<'a> {
    /// The operation type (e.g., "subscribe").
    ///
    /// Specifies the WebSocket operation, typically `"subscribe"` for subscribing to data streams. Bots must set this correctly to initiate subscriptions.
    pub op: &'a str,

    /// A list of subscription arguments.
    ///
    /// Specifies the data streams to subscribe to, such as `"orderbook.50.BTCUSDT"` or `"trade.BTCUSDT"`. Bots should provide valid topics to receive relevant market data.
    pub args: Vec<&'a str>,
}

impl<'a> Subscription<'a> {
    /// Constructs a new Subscription with specified parameters.
    ///
    /// Allows customization of the WebSocket subscription. Bots should use this to specify the operation and subscription arguments for their data needs.
    pub fn new(op: &'a str, args: Vec<&'a str>) -> Self {
        Self { op, args }
    }
    /// Creates a default Subscription.
    ///
    /// Returns a subscription with `op` set to `"subscribe"` and an empty argument list. Suitable for testing but should be customized with valid topics for production.
    pub fn default() -> Subscription<'a> {
        Self::new("subscribe", vec![])
    }

    /// Returns a new Subscription with `op` set to `"unsubscribe"`.
    ///
    /// The returned subscription will have the same `args` as the original, allowing for easy unsubscription of all topics.
    pub fn unsubscribe(&self) -> Subscription<'a> {
        Self::new("unsubscribe", self.args.clone())
    }
}
