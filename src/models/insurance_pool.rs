use crate::prelude::*;

/// Represents a single insurance pool entry in the WebSocket stream.
///
/// Contains information about the insurance pool balance for a specific symbol or group of symbols.
/// Insurance pools are used to cover losses when a trader's position is liquidated below the
/// bankruptcy price, preventing auto-deleveraging of other traders.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePool {
    /// The coin/token of the insurance pool (e.g., "USDT", "USDC", "BTC").
    ///
    /// Specifies the currency used for the insurance pool.
    /// For inverse contracts, this would be the base coin (e.g., "BTC" for BTCUSD).
    pub coin: String,

    /// The symbol(s) associated with this insurance pool.
    ///
    /// For isolated insurance pools, this is a single symbol name (e.g., "BTCUSDT").
    /// For shared insurance pools, this may contain multiple symbols or be a group identifier.
    /// Note: Shared insurance pool data is not pushed via WebSocket.
    pub symbols: String,

    /// The current balance of the insurance pool in the specified coin.
    ///
    /// A positive balance indicates the fund has surplus to cover losses.
    /// A negative balance indicates the fund is depleted and may trigger ADL (Auto-Deleveraging).
    #[serde(with = "string_to_float")]
    pub balance: f64,

    /// The timestamp when this insurance pool data was last updated (in milliseconds).
    ///
    /// Indicates when the balance information was refreshed by Bybit's systems.
    /// For shared insurance pools, this field follows a T+1 refresh mechanism
    /// and is updated daily at 00:00 UTC.
    #[serde(rename = "updateTime")]
    #[serde(with = "string_to_u64")]
    pub update_time: u64,
}

impl InsurancePool {
    /// Constructs a new InsurancePool with specified parameters.
    pub fn new(coin: &str, symbols: &str, balance: f64, update_time: u64) -> Self {
        Self {
            coin: coin.to_string(),
            symbols: symbols.to_string(),
            balance,
            update_time,
        }
    }

    /// Returns true if this insurance pool is for a specific coin.
    pub fn is_coin(&self, coin: &str) -> bool {
        self.coin.eq_ignore_ascii_case(coin)
    }

    /// Returns true if this insurance pool is for a specific symbol.
    ///
    /// Note: For shared insurance pools, the symbols field may contain multiple symbols
    /// or a group identifier, so this check may not be accurate for all cases.
    pub fn is_symbol(&self, symbol: &str) -> bool {
        self.symbols.eq_ignore_ascii_case(symbol)
    }

    /// Returns true if the insurance pool balance is positive.
    pub fn is_positive(&self) -> bool {
        self.balance > 0.0
    }

    /// Returns true if the insurance pool balance is negative or zero.
    ///
    /// A non-positive balance may indicate that ADL (Auto-Deleveraging) could be triggered.
    pub fn is_non_positive(&self) -> bool {
        self.balance <= 0.0
    }

    /// Returns the update time as a chrono DateTime.
    pub fn update_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.update_time / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the time since the last update in seconds.
    pub fn time_since_update(&self) -> u64 {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        if now > self.update_time {
            (now - self.update_time) / 1000
        } else {
            0
        }
    }

    /// Checks if the data is stale (older than 5 minutes).
    ///
    /// The WebSocket pushes updates every 1 second, so data older than 5 minutes
    /// might be considered stale for real-time trading decisions.
    pub fn is_stale(&self) -> bool {
        self.time_since_update() > 300
    }

    /// Returns the absolute value of the balance.
    ///
    /// Useful for display purposes when the sign indicates deficit/surplus.
    pub fn absolute_balance(&self) -> f64 {
        self.balance.abs()
    }

    /// Returns a string representation of the balance with sign.
    ///
    /// Positive balances show "+" prefix, negative balances show "-" prefix.
    pub fn signed_balance_string(&self) -> String {
        if self.balance >= 0.0 {
            format!("+{:.8}", self.balance)
        } else {
            format!("{:.8}", self.balance)
        }
    }
}

/// Represents a WebSocket insurance pool update event.
///
/// Contains real-time updates to insurance pool balances for various symbols.
/// Push frequency: 1 second for USDT contracts, USDC contracts, and inverse contracts.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePoolUpdate {
    /// The WebSocket topic for the event (e.g., "insurance.USDT", "insurance.USDC", "insurance.inverse").
    ///
    /// Specifies the data stream for the insurance pool update.
    /// Bots use this to determine which contract group the update belongs to.
    #[serde(rename = "topic")]
    pub topic: String,

    /// The event type (e.g., "snapshot", "delta").
    ///
    /// Snapshot contains the full current state, delta contains incremental changes.
    /// Bots should use snapshot to initialize state and delta to update it.
    #[serde(rename = "type")]
    pub event_type: String,

    /// The timestamp of the event in milliseconds.
    ///
    /// Indicates when the insurance pool update was generated by the system.
    /// Bots use this to ensure data freshness and time-based analysis.
    #[serde(rename = "ts")]
    pub timestamp: u64,

    /// The insurance pool data.
    ///
    /// Contains a list of insurance pool entries. Each entry represents the balance
    /// for a specific symbol or group of symbols.
    /// No event will be published if all insurance pool balances remain unchanged.
    #[serde(rename = "data")]
    pub data: Vec<InsurancePool>,
}

impl InsurancePoolUpdate {
    /// Returns the contract group from the topic.
    ///
    /// Extracts the contract group identifier from the WebSocket topic.
    /// Examples:
    /// - "insurance.USDT" -> "USDT"
    /// - "insurance.USDC" -> "USDC"
    /// - "insurance.inverse" -> "inverse"
    pub fn contract_group(&self) -> Option<&str> {
        self.topic.split('.').next_back()
    }

    /// Returns true if this is a snapshot update.
    ///
    /// Snapshot updates contain the full insurance pool state and should replace
    /// the local state for the corresponding contract group.
    pub fn is_snapshot(&self) -> bool {
        self.event_type == "snapshot"
    }

    /// Returns true if this is a delta update.
    ///
    /// Delta updates contain incremental changes and should be applied to
    /// the local insurance pool state.
    pub fn is_delta(&self) -> bool {
        self.event_type == "delta"
    }

    /// Returns the timestamp as a chrono DateTime.
    pub fn timestamp_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.timestamp / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the number of insurance pool entries in this update.
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Returns true if there are no insurance pool entries in this update.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Finds an insurance pool entry for a specific symbol.
    ///
    /// Returns the first matching insurance pool entry for the given symbol.
    /// Note: For shared insurance pools, the symbols field may contain multiple
    /// symbols or a group identifier, so this may not find all relevant entries.
    pub fn find_by_symbol(&self, symbol: &str) -> Option<&InsurancePool> {
        self.data.iter().find(|pool| pool.is_symbol(symbol))
    }

    /// Finds all insurance pool entries for a specific coin.
    pub fn filter_by_coin(&self, coin: &str) -> Vec<&InsurancePool> {
        self.data.iter().filter(|pool| pool.is_coin(coin)).collect()
    }

    /// Returns the total balance across all insurance pools in this update.
    ///
    /// Sums the balances of all insurance pool entries in this update.
    /// Useful for monitoring the overall health of a contract group's insurance.
    pub fn total_balance(&self) -> f64 {
        self.data.iter().map(|pool| pool.balance).sum()
    }

    /// Returns the number of insurance pools with positive balance.
    pub fn count_positive(&self) -> usize {
        self.data.iter().filter(|pool| pool.is_positive()).count()
    }

    /// Returns the number of insurance pools with non-positive balance.
    ///
    /// These are pools that may be at risk of triggering ADL (Auto-Deleveraging).
    pub fn count_non_positive(&self) -> usize {
        self.data
            .iter()
            .filter(|pool| pool.is_non_positive())
            .count()
    }

    /// Returns the minimum balance among all insurance pools.
    pub fn min_balance(&self) -> Option<f64> {
        self.data.iter().map(|pool| pool.balance).reduce(f64::min)
    }

    /// Returns the maximum balance among all insurance pools.
    pub fn max_balance(&self) -> Option<f64> {
        self.data.iter().map(|pool| pool.balance).reduce(f64::max)
    }

    /// Returns the average balance across all insurance pools.
    pub fn average_balance(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.total_balance() / self.data.len() as f64)
        }
    }

    /// Returns all insurance pools that are stale (older than 5 minutes).
    ///
    /// Note: The WebSocket pushes updates every 1 second when changes occur,
    /// so stale entries in a fresh update may indicate issues with specific symbols.
    pub fn stale_pools(&self) -> Vec<&InsurancePool> {
        self.data.iter().filter(|pool| pool.is_stale()).collect()
    }

    /// Returns the time of the most recent update among all insurance pools.
    pub fn latest_update_time(&self) -> Option<u64> {
        self.data.iter().map(|pool| pool.update_time).max()
    }

    /// Returns the time of the oldest update among all insurance pools.
    pub fn earliest_update_time(&self) -> Option<u64> {
        self.data.iter().map(|pool| pool.update_time).min()
    }
}
