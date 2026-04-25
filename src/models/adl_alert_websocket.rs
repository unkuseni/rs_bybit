use crate::prelude::*;

/// Represents an ADL (Auto-Deleveraging) alert item in a WebSocket stream.
///
/// ADL is a risk management mechanism that automatically closes positions
/// when the insurance pool balance reaches certain thresholds to prevent
/// systemic risk. This struct is used in WebSocket streams to provide
/// real-time ADL alert information.
///
/// # Bybit API Reference
/// The Bybit WebSocket API (https://bybit-exchange.github.io/docs/v5/websocket/public/adl-alert)
/// provides ADL alert updates with a push frequency of 1 second.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ADLAlertWebsocketItem {
    /// The token of the insurance pool (e.g., "USDT", "USDC").
    /// Specifies the currency used for the insurance pool.
    #[serde(rename = "c")]
    pub coin: String,

    /// The trading pair name (e.g., "BTCUSDT").
    /// Identifies the contract for which the ADL alert applies.
    #[serde(rename = "s")]
    pub symbol: String,

    /// The balance of the insurance fund.
    /// Used to determine if ADL is triggered. For shared insurance pools,
    /// this field follows a T+1 refresh mechanism and is updated daily at 00:00 UTC.
    /// When balance ≤ 0, insurance pool equity ADL is triggered.
    #[serde(rename = "b")]
    #[serde(with = "string_to_float")]
    pub balance: f64,

    /// The maximum balance of the insurance pool in the last 8 hours.
    /// Note: According to the API documentation, this field is deprecated and always returns "".
    /// It's included for compatibility but should not be relied upon.
    #[serde(rename = "mb")]
    #[serde(with = "string_to_float")]
    pub max_balance: f64,

    /// The PnL ratio threshold for triggering contract PnL drawdown ADL.
    /// ADL is triggered when the symbol's PnL drawdown ratio in the last 8 hours
    /// exceeds this value. Typically a negative value like "-0.3".
    #[serde(rename = "i_pr")]
    #[serde(with = "string_to_float")]
    pub insurance_pnl_ratio: f64,

    /// The symbol's PnL drawdown ratio in the last 8 hours.
    /// Used to determine whether ADL is triggered or stopped.
    /// Calculated as: (Symbol's current PnL - Symbol's 8h max PnL) / Insurance pool's 8h max balance.
    #[serde(rename = "pr")]
    #[serde(with = "string_to_float")]
    pub pnl_ratio: f64,

    /// The trigger threshold for contract PnL drawdown ADL.
    /// This condition is only effective when the insurance pool balance is greater than this value.
    /// If so, an 8-hour drawdown exceeding the insurance_pnl_ratio may trigger ADL.
    /// Typically a value like "10000".
    #[serde(rename = "adl_tt")]
    #[serde(with = "string_to_float")]
    pub adl_trigger_threshold: f64,

    /// The stop ratio threshold for contract PnL drawdown ADL.
    /// ADL stops when the symbol's 8-hour drawdown ratio falls below this value.
    /// Typically a value like "-0.25".
    #[serde(rename = "adl_sr")]
    #[serde(with = "string_to_float")]
    pub adl_stop_ratio: f64,
}

impl ADLAlertWebsocketItem {
    /// Constructs a new ADLAlertWebsocketItem with specified parameters.
    pub fn new(
        coin: &str,
        symbol: &str,
        balance: f64,
        max_balance: f64,
        insurance_pnl_ratio: f64,
        pnl_ratio: f64,
        adl_trigger_threshold: f64,
        adl_stop_ratio: f64,
    ) -> Self {
        Self {
            coin: coin.to_string(),
            symbol: symbol.to_string(),
            balance,
            max_balance,
            insurance_pnl_ratio,
            pnl_ratio,
            adl_trigger_threshold,
            adl_stop_ratio,
        }
    }

    /// Returns true if this ADL alert item is for a specific coin.
    pub fn is_coin(&self, coin: &str) -> bool {
        self.coin.eq_ignore_ascii_case(coin)
    }

    /// Returns true if this ADL alert item is for a specific symbol.
    pub fn is_symbol(&self, symbol: &str) -> bool {
        self.symbol.eq_ignore_ascii_case(symbol)
    }

    /// Checks if contract PnL drawdown ADL should be triggered.
    /// According to the API documentation, ADL is triggered when:
    /// 1. `balance` > `adl_trigger_threshold`
    /// 2. `pnl_ratio` < `insurance_pnl_ratio`
    pub fn is_contract_pnl_drawdown_adl_triggered(&self) -> bool {
        self.balance > self.adl_trigger_threshold && self.pnl_ratio < self.insurance_pnl_ratio
    }

    /// Checks if insurance pool equity ADL should be triggered.
    /// According to the API documentation, ADL is triggered when:
    /// `balance` ≤ 0
    pub fn is_insurance_pool_equity_adl_triggered(&self) -> bool {
        self.balance <= 0.0
    }

    /// Checks if contract PnL drawdown ADL should be stopped.
    /// According to the API documentation, ADL stops when:
    /// `pnl_ratio` > `adl_stop_ratio`
    pub fn is_contract_pnl_drawdown_adl_stopped(&self) -> bool {
        self.pnl_ratio > self.adl_stop_ratio
    }

    /// Checks if insurance pool equity ADL should be stopped.
    /// According to the API documentation, ADL stops when:
    /// `balance` > 0
    pub fn is_insurance_pool_equity_adl_stopped(&self) -> bool {
        self.balance > 0.0
    }

    /// Returns the ADL status for this item.
    /// Returns a tuple of (contract_triggered, contract_stopped, equity_triggered, equity_stopped).
    pub fn adl_status(&self) -> (bool, bool, bool, bool) {
        (
            self.is_contract_pnl_drawdown_adl_triggered(),
            self.is_contract_pnl_drawdown_adl_stopped(),
            self.is_insurance_pool_equity_adl_triggered(),
            self.is_insurance_pool_equity_adl_stopped(),
        )
    }

    /// Returns true if any ADL condition is currently triggered.
    pub fn is_any_adl_triggered(&self) -> bool {
        self.is_contract_pnl_drawdown_adl_triggered()
            || self.is_insurance_pool_equity_adl_triggered()
    }

    /// Returns true if all ADL conditions are stopped.
    pub fn is_all_adl_stopped(&self) -> bool {
        self.is_contract_pnl_drawdown_adl_stopped() && self.is_insurance_pool_equity_adl_stopped()
    }

    /// Returns the absolute value of the balance.
    pub fn absolute_balance(&self) -> f64 {
        self.balance.abs()
    }

    /// Returns a string representation of the balance with sign.
    pub fn signed_balance_string(&self) -> String {
        if self.balance >= 0.0 {
            format!("+{:.8}", self.balance)
        } else {
            format!("{:.8}", self.balance)
        }
    }

    /// Returns the drawdown amount relative to the insurance PnL ratio threshold.
    pub fn drawdown_amount(&self) -> f64 {
        self.insurance_pnl_ratio - self.pnl_ratio
    }

    /// Returns true if the drawdown exceeds the threshold.
    pub fn is_drawdown_exceeding_threshold(&self) -> bool {
        self.drawdown_amount() > 0.0
    }

    /// Returns the safety margin before ADL trigger.
    pub fn safety_margin(&self) -> f64 {
        if self.balance > self.adl_trigger_threshold {
            self.balance - self.adl_trigger_threshold
        } else {
            0.0
        }
    }

    /// Returns the safety margin as a percentage of trigger threshold.
    pub fn safety_margin_percentage(&self) -> f64 {
        if self.adl_trigger_threshold > 0.0 {
            self.safety_margin() / self.adl_trigger_threshold * 100.0
        } else {
            0.0
        }
    }

    /// Returns a summary string for this ADL alert item.
    pub fn to_summary_string(&self) -> String {
        let (contract_triggered, contract_stopped, equity_triggered, equity_stopped) =
            self.adl_status();

        format!(
            "{} {}: Balance={:.2}, PnL Ratio={:.4}%, Safety={:.2}%, ContractADL={}/{}, EquityADL={}/{}",
            self.coin,
            self.symbol,
            self.balance,
            self.pnl_ratio * 100.0,
            self.safety_margin_percentage(),
            if contract_triggered { "TRIGGERED" } else { "ok" },
            if contract_stopped { "STOPPED" } else { "active" },
            if equity_triggered { "TRIGGERED" } else { "ok" },
            if equity_stopped { "STOPPED" } else { "active" }
        )
    }
}

/// Represents a WebSocket ADL alert update event.
///
/// Contains real-time ADL alert information for various trading pairs.
/// Push frequency: 1 second for USDT Perpetual/Delivery, USDC Perpetual/Delivery, and Inverse Contracts.
///
/// # Bybit API Reference
/// Topic: `adlAlert.{coin}` where coin can be:
/// - `adlAlert.USDT` for USDT Perpetual/Delivery
/// - `adlAlert.USDC` for USDC Perpetual/Delivery
/// - `adlAlert.inverse` for Inverse contracts
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ADLAlertUpdate {
    /// The WebSocket topic for the event (e.g., "adlAlert.USDT", "adlAlert.USDC", "adlAlert.inverse").
    ///
    /// Specifies the data stream for the ADL alert update.
    /// Bots use this to determine which contract group the update belongs to.
    #[serde(rename = "topic")]
    pub topic: String,

    /// The event type (e.g., "snapshot").
    ///
    /// ADL alert updates are typically snapshot type, containing the full current state.
    #[serde(rename = "type")]
    pub event_type: String,

    /// The timestamp of the event in milliseconds.
    ///
    /// Indicates when the ADL alert update was generated by the system.
    /// Bots use this to ensure data freshness and time-based analysis.
    #[serde(rename = "ts")]
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,

    /// The ADL alert data.
    ///
    /// Contains a list of ADL alert items. Each item represents ADL alert information
    /// for a specific trading pair.
    #[serde(rename = "data")]
    pub data: Vec<ADLAlertWebsocketItem>,
}

impl ADLAlertUpdate {
    /// Returns the contract group from the topic.
    ///
    /// Extracts the contract group identifier from the WebSocket topic.
    /// Examples:
    /// - "adlAlert.USDT" -> "USDT"
    /// - "adlAlert.USDC" -> "USDC"
    /// - "adlAlert.inverse" -> "inverse"
    pub fn contract_group(&self) -> Option<&str> {
        self.topic.split('.').next_back()
    }

    /// Returns true if this is a snapshot update.
    ///
    /// Snapshot updates contain the full ADL alert state and should replace
    /// the local state for the corresponding contract group.
    pub fn is_snapshot(&self) -> bool {
        self.event_type == "snapshot"
    }

    /// Returns the timestamp as a chrono DateTime.
    pub fn timestamp_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.timestamp / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the age of the update in milliseconds.
    ///
    /// Calculates how old this update is relative to the current time.
    pub fn age_ms(&self) -> u64 {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        now.saturating_sub(self.timestamp)
    }

    /// Returns true if the update is stale (older than 2 seconds).
    ///
    /// Since ADL alert updates are pushed every 1 second, data older than 2 seconds
    /// might be considered stale for real-time trading decisions.
    pub fn is_stale(&self) -> bool {
        self.age_ms() > 2000
    }

    /// Returns the number of ADL alert items in this update.
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Returns true if there are no ADL alert items in this update.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Finds an ADL alert item for a specific symbol.
    ///
    /// Returns the first matching ADL alert item for the given symbol.
    pub fn find_by_symbol(&self, symbol: &str) -> Option<&ADLAlertWebsocketItem> {
        self.data.iter().find(|item| item.is_symbol(symbol))
    }

    /// Finds all ADL alert items for a specific coin.
    pub fn filter_by_coin(&self, coin: &str) -> Vec<&ADLAlertWebsocketItem> {
        self.data.iter().filter(|item| item.is_coin(coin)).collect()
    }

    /// Returns all ADL alert items where any ADL condition is triggered.
    pub fn triggered_items(&self) -> Vec<&ADLAlertWebsocketItem> {
        self.data
            .iter()
            .filter(|item| item.is_any_adl_triggered())
            .collect()
    }

    /// Returns all ADL alert items where all ADL conditions are stopped.
    pub fn stopped_items(&self) -> Vec<&ADLAlertWebsocketItem> {
        self.data
            .iter()
            .filter(|item| item.is_all_adl_stopped())
            .collect()
    }

    /// Returns the number of ADL alert items with triggered conditions.
    pub fn count_triggered(&self) -> usize {
        self.triggered_items().len()
    }

    /// Returns the number of ADL alert items with stopped conditions.
    pub fn count_stopped(&self) -> usize {
        self.stopped_items().len()
    }

    /// Returns the total balance across all ADL alert items.
    pub fn total_balance(&self) -> f64 {
        self.data.iter().map(|item| item.balance).sum()
    }

    /// Returns the average PnL ratio across all ADL alert items.
    pub fn average_pnl_ratio(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.iter().map(|item| item.pnl_ratio).sum::<f64>() / self.data.len() as f64)
        }
    }

    /// Returns the minimum balance among all ADL alert items.
    pub fn min_balance(&self) -> Option<f64> {
        self.data.iter().map(|item| item.balance).reduce(f64::min)
    }

    /// Returns the maximum balance among all ADL alert items.
    pub fn max_balance(&self) -> Option<f64> {
        self.data.iter().map(|item| item.balance).reduce(f64::max)
    }

    /// Returns the item with the lowest balance (most at risk).
    pub fn most_at_risk_item(&self) -> Option<&ADLAlertWebsocketItem> {
        self.data.iter().min_by(|a, b| {
            a.balance
                .partial_cmp(&b.balance)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns the item with the highest balance (least at risk).
    pub fn least_at_risk_item(&self) -> Option<&ADLAlertWebsocketItem> {
        self.data.iter().max_by(|a, b| {
            a.balance
                .partial_cmp(&b.balance)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns a summary string for this ADL alert update.
    pub fn to_summary_string(&self) -> String {
        format!(
            "[{}] {}: {} items, {} triggered, {} stopped, Total Balance={:.2}, Avg PnL={:.4}%",
            self.timestamp_datetime().format("%H:%M:%S"),
            self.topic,
            self.count(),
            self.count_triggered(),
            self.count_stopped(),
            self.total_balance(),
            self.average_pnl_ratio().unwrap_or(0.0) * 100.0
        )
    }

    /// Validates the update for trading use.
    ///
    /// Returns `true` if:
    /// 1. The update is not stale (≤ 2 seconds old)
    /// 2. There is at least one ADL alert item
    /// 3. The contract group can be extracted from the topic
    pub fn is_valid_for_trading(&self) -> bool {
        !self.is_stale() && !self.data.is_empty() && self.contract_group().is_some()
    }

    /// Returns the update latency in milliseconds.
    ///
    /// For comparing with other market data timestamps.
    pub fn latency_ms(&self, other_timestamp: u64) -> i64 {
        if self.timestamp > other_timestamp {
            (self.timestamp - other_timestamp) as i64
        } else {
            (other_timestamp - self.timestamp) as i64
        }
    }

    /// Returns all symbols that have ADL conditions triggered.
    pub fn triggered_symbols(&self) -> Vec<String> {
        self.triggered_items()
            .iter()
            .map(|item| item.symbol.clone())
            .collect()
    }

    /// Returns all coins that have ADL conditions triggered.
    pub fn triggered_coins(&self) -> Vec<String> {
        let mut coins = std::collections::HashSet::new();
        for item in self.triggered_items() {
            coins.insert(item.coin.clone());
        }
        coins.into_iter().collect()
    }
}
