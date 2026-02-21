use serde::{Deserialize, Serialize};

/// Request for getting coin information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinInfoRequest<'a> {
    /// Coin, uppercase only
    pub coin: Option<&'a str>,
}
