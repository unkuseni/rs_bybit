use crate::prelude::*;

/// Represents a single RPI (Real-time Price Improvement) order book level.
///
/// Each level contains the price, non-RPI size, and RPI size for either bids or asks.
/// RPI orders are special orders that can improve prices for takers.
#[derive(Clone, Debug)]
pub struct RPIOrderbookLevel {
    /// The price level.
    pub price: f64,

    /// The non-RPI size at this price level.
    ///
    /// This represents the regular order quantity at this price.
    /// When delta data has size=0, it means all quotations for this price have been filled or cancelled.
    pub non_rpi_size: f64,

    /// The RPI size at this price level.
    ///
    /// This represents the RPI (Real-time Price Improvement) order quantity at this price.
    /// When a bid RPI order crosses with a non-RPI ask price, the quantity of the bid RPI becomes invalid and is hidden.
    /// When an ask RPI order crosses with a non-RPI bid price, the quantity of the ask RPI becomes invalid and is hidden.
    pub rpi_size: f64,
}

impl<'de> Deserialize<'de> for RPIOrderbookLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize as an array of 3 strings
        let arr: [String; 3] = Deserialize::deserialize(deserializer)?;

        let price = arr[0].parse::<f64>().map_err(serde::de::Error::custom)?;
        let non_rpi_size = arr[1].parse::<f64>().map_err(serde::de::Error::custom)?;
        let rpi_size = arr[2].parse::<f64>().map_err(serde::de::Error::custom)?;

        Ok(Self {
            price,
            non_rpi_size,
            rpi_size,
        })
    }
}

impl Serialize for RPIOrderbookLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as an array of 3 strings
        let arr = [
            self.price.to_string(),
            self.non_rpi_size.to_string(),
            self.rpi_size.to_string(),
        ];
        arr.serialize(serializer)
    }
}

impl RPIOrderbookLevel {
    /// Constructs a new RPIOrderbookLevel with specified price, non-RPI size, and RPI size.
    pub fn new(price: f64, non_rpi_size: f64, rpi_size: f64) -> Self {
        Self {
            price,
            non_rpi_size,
            rpi_size,
        }
    }

    /// Returns the total size (non-RPI + RPI) at this price level.
    pub fn total_size(&self) -> f64 {
        self.non_rpi_size + self.rpi_size
    }

    /// Returns true if this level has any RPI size.
    pub fn has_rpi(&self) -> bool {
        self.rpi_size > 0.0
    }

    /// Returns true if this level has any non-RPI size.
    pub fn has_non_rpi(&self) -> bool {
        self.non_rpi_size > 0.0
    }
}

/// Represents the RPI (Real-time Price Improvement) order book for a trading pair.
///
/// Contains the current bid and ask levels with RPI information, along with metadata.
/// RPI order books show both regular orders and RPI orders, which can provide price improvement.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RPIOrderbook {
    /// The trading pair symbol (e.g., "BTCUSDT").
    #[serde(rename = "s")]
    pub symbol: String,

    /// A list of ask (sell) orders with RPI information.
    ///
    /// Each element is an array of [price, non-RPI size, RPI size].
    /// Sorted by price in ascending order.
    #[serde(rename = "a")]
    pub asks: Vec<RPIOrderbookLevel>,

    /// A list of bid (buy) orders with RPI information.
    ///
    /// Each element is an array of [price, non-RPI size, RPI size].
    /// Sorted by price in descending order.
    #[serde(rename = "b")]
    pub bids: Vec<RPIOrderbookLevel>,

    /// The timestamp (ms) that the system generates the data.
    #[serde(rename = "ts")]
    pub timestamp: u64,

    /// Update ID, is always in sequence corresponds to `u` in the 50-level WebSocket RPI orderbook stream.
    #[serde(rename = "u")]
    pub update_id: u64,

    /// Cross sequence.
    ///
    /// You can use this field to compare different levels orderbook data, and for the smaller seq,
    /// then it means the data is generated earlier.
    #[serde(rename = "seq")]
    pub sequence: u64,

    /// The timestamp from the matching engine when this orderbook data is produced.
    /// It can be correlated with `T` from public trade channel.
    #[serde(rename = "cts")]
    pub matching_engine_timestamp: u64,
}

impl RPIOrderbook {
    /// Returns the best ask price (lowest ask).
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|ask| ask.price)
    }

    /// Returns the best bid price (highest bid).
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|bid| bid.price)
    }

    /// Returns the bid-ask spread.
    pub fn spread(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Returns the mid price (average of best bid and ask).
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }

    /// Returns the total RPI size on the ask side.
    pub fn total_ask_rpi_size(&self) -> f64 {
        self.asks.iter().map(|ask| ask.rpi_size).sum()
    }

    /// Returns the total non-RPI size on the ask side.
    pub fn total_ask_non_rpi_size(&self) -> f64 {
        self.asks.iter().map(|ask| ask.non_rpi_size).sum()
    }

    /// Returns the total RPI size on the bid side.
    pub fn total_bid_rpi_size(&self) -> f64 {
        self.bids.iter().map(|bid| bid.rpi_size).sum()
    }

    /// Returns the total non-RPI size on the bid side.
    pub fn total_bid_non_rpi_size(&self) -> f64 {
        self.bids.iter().map(|bid| bid.non_rpi_size).sum()
    }

    /// Returns the total size (RPI + non-RPI) on the ask side.
    pub fn total_ask_size(&self) -> f64 {
        self.asks.iter().map(|ask| ask.total_size()).sum()
    }

    /// Returns the total size (RPI + non-RPI) on the bid side.
    pub fn total_bid_size(&self) -> f64 {
        self.bids.iter().map(|bid| bid.total_size()).sum()
    }
}
