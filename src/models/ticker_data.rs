use crate::prelude::*;

/// Enum representing ticker data for different instrument types.
///
/// This untagged enum allows the API to return ticker data for either spot, futures (including perpetuals), or options.
/// For perpetual futures, the `Futures` variant is most relevant, containing funding rate and open interest data.
/// For options, the `Options` variant contains Greeks and implied volatility data.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TickerData {
    /// Ticker data for spot markets.
    ///
    /// Contains market data for spot trading pairs. Not relevant for perpetual futures.
    Spot(SpotTicker),

    /// Ticker data for futures (including perpetuals).
    ///
    /// Contains market data for perpetual futures, including funding rates and open interest. Critical for bots monitoring market conditions and funding costs.
    Futures(FuturesTicker),

    /// Ticker data for options contracts.
    ///
    /// Contains market data for options contracts, including Greeks (delta, gamma, vega, theta),
    /// implied volatility, and other options-specific metrics.
    Options(OptionsTicker),
}
