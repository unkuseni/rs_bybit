use derive_more::TryUnwrap;

use crate::prelude::*;

/// Enum representing ticker data for different market types.
///
/// Encapsulates ticker data for linear perpetuals, spot markets, options, and futures contracts, allowing bots to process market-specific metrics like funding rates, USD index prices, or options Greeks. Bots use this to handle ticker updates in a type-safe manner.
#[derive(Debug, Serialize, Deserialize, Clone, TryUnwrap)]
#[serde(untagged)]
pub enum Ticker {
    /// Ticker data for linear perpetual futures.
    ///
    /// Contains metrics like funding rate and open interest for USDT-margined contracts. Bots use this for perpetual futures trading strategies.
    Linear(LinearTickerData),

    /// Ticker data for spot markets.
    ///
    /// Contains metrics like 24-hour volume and USD index price for spot trading pairs. Bots use this for spot market analysis.
    Spot(SpotTickerData),

    /// Ticker data for options contracts.
    ///
    /// Contains metrics like Greeks (delta, gamma, vega, theta), implied volatility, and other options-specific data. Bots use this for options trading strategies.
    Options(OptionsTicker),

    /// Ticker data for futures contracts (including inverse and USDC futures).
    ///
    /// Contains metrics for futures contracts with delivery dates. Bots use this for futures trading strategies.
    Futures(FuturesTicker),
}
