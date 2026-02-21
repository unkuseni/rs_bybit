use serde::{Deserialize, Serialize};

/// Delivery record item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliveryRecordItem {
    /// Delivery time (ms)
    #[serde(rename = "deliveryTime")]
    pub delivery_time: String,
    /// Symbol name
    pub symbol: String,
    /// Side: `Buy`,`Sell`
    pub side: String,
    /// Executed size
    pub position: String,
    /// Avg entry price
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    /// Delivery price
    #[serde(rename = "deliveryPrice")]
    pub delivery_price: String,
    /// Exercise price (for options)
    pub strike: String,
    /// Trading fee
    pub fee: String,
    /// Realized PnL of the delivery
    #[serde(rename = "deliveryRpl")]
    pub delivery_rpl: String,
}
