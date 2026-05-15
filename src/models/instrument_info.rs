use crate::prelude::*;

/// Enum representing different types of instrument information.
///
/// This untagged enum allows the API to return different instrument details based on the category (Futures, Spot, Options). For perpetual futures, the `Futures` variant is most relevant, containing details like leverage and funding intervals.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum InstrumentInfo {
    /// Instrument information for futures (including perpetuals).
    ///
    /// Contains details for perpetual futures contracts, such as leverage filters and funding intervals. Bots use this to configure trading parameters and manage risk.
    Futures(FuturesInstrumentsInfo),

    /// Instrument information for spot markets.
    ///
    /// Contains details for spot trading pairs. Less relevant for perpetual futures but included for completeness.
    Spot(SpotInstrumentsInfo),

    /// Instrument information for options.
    ///
    /// Contains details for options contracts. Not typically used for perpetual futures trading.
    Options(Box<OptionsInstrument>),
}
