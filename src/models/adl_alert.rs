use crate::prelude::*;

/// Represents an ADL (Auto-Deleveraging) alert for a trading pair.
/// ADL is a risk management mechanism that automatically closes positions
/// when the insurance pool balance reaches certain thresholds to prevent
/// systemic risk.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ADLAlertItem {
    /// The token of the insurance pool (e.g., "USDT", "USDC").
    /// Specifies the currency used for the insurance pool.
    pub coin: String,

    /// The trading pair name (e.g., "BTCUSDT").
    /// Identifies the contract for which the ADL alert applies.
    pub symbol: String,

    /// The balance of the insurance fund.
    /// Used to determine if ADL is triggered. For shared insurance pools,
    /// this field follows a T+1 refresh mechanism and is updated daily at 00:00 UTC.
    /// When balance ≤ 0, insurance pool equity ADL is triggered.
    pub balance: String,

    /// The maximum balance of the insurance pool in the last 8 hours.
    /// Note: According to the API documentation, this field is deprecated and always returns "".
    /// It's included for compatibility but should not be relied upon.
    pub max_balance: String,

    /// The PnL ratio threshold for triggering contract PnL drawdown ADL.
    /// ADL is triggered when the symbol's PnL drawdown ratio in the last 8 hours
    /// exceeds this value. Typically a negative value like "-0.3".
    pub insurance_pnl_ratio: String,

    /// The symbol's PnL drawdown ratio in the last 8 hours.
    /// Used to determine whether ADL is triggered or stopped.
    /// Calculated as: (Symbol's current PnL - Symbol's 8h max PnL) / Insurance pool's 8h max balance.
    pub pnl_ratio: String,

    /// The trigger threshold for contract PnL drawdown ADL.
    /// This condition is only effective when the insurance pool balance is greater than this value.
    /// If so, an 8-hour drawdown exceeding the insurance_pnl_ratio may trigger ADL.
    /// Typically a value like "10000".
    pub adl_trigger_threshold: String,

    /// The stop ratio threshold for contract PnL drawdown ADL.
    /// ADL stops when the symbol's 8-hour drawdown ratio falls below this value.
    /// Typically a value like "-0.25".
    pub adl_stop_ratio: String,
}

impl ADLAlertItem {
    /// Constructs a new ADLAlertItem with specified parameters.
    pub fn new(
        coin: &str,
        symbol: &str,
        balance: &str,
        max_balance: &str,
        insurance_pnl_ratio: &str,
        pnl_ratio: &str,
        adl_trigger_threshold: &str,
        adl_stop_ratio: &str,
    ) -> Self {
        Self {
            coin: coin.to_string(),
            symbol: symbol.to_string(),
            balance: balance.to_string(),
            max_balance: max_balance.to_string(),
            insurance_pnl_ratio: insurance_pnl_ratio.to_string(),
            pnl_ratio: pnl_ratio.to_string(),
            adl_trigger_threshold: adl_trigger_threshold.to_string(),
            adl_stop_ratio: adl_stop_ratio.to_string(),
        }
    }

    /// Returns the balance as a floating-point number.
    /// Returns `None` if the balance cannot be parsed.
    pub fn balance_as_f64(&self) -> Option<f64> {
        self.balance.parse::<f64>().ok()
    }

    /// Returns the insurance PnL ratio as a floating-point number.
    /// Returns `None` if the ratio cannot be parsed.
    pub fn insurance_pnl_ratio_as_f64(&self) -> Option<f64> {
        self.insurance_pnl_ratio.parse::<f64>().ok()
    }

    /// Returns the PnL ratio as a floating-point number.
    /// Returns `None` if the ratio cannot be parsed.
    pub fn pnl_ratio_as_f64(&self) -> Option<f64> {
        self.pnl_ratio.parse::<f64>().ok()
    }

    /// Returns the ADL trigger threshold as a floating-point number.
    /// Returns `None` if the threshold cannot be parsed.
    pub fn adl_trigger_threshold_as_f64(&self) -> Option<f64> {
        self.adl_trigger_threshold.parse::<f64>().ok()
    }

    /// Returns the ADL stop ratio as a floating-point number.
    /// Returns `None` if the ratio cannot be parsed.
    pub fn adl_stop_ratio_as_f64(&self) -> Option<f64> {
        self.adl_stop_ratio.parse::<f64>().ok()
    }

    /// Checks if contract PnL drawdown ADL should be triggered.
    /// According to the API documentation, ADL is triggered when:
    /// 1. `balance` > `adl_trigger_threshold`
    /// 2. `pnl_ratio` < `insurance_pnl_ratio`
    pub fn is_contract_pnl_drawdown_adl_triggered(&self) -> Option<bool> {
        let balance = self.balance_as_f64()?;
        let trigger_threshold = self.adl_trigger_threshold_as_f64()?;
        let pnl_ratio = self.pnl_ratio_as_f64()?;
        let insurance_pnl_ratio = self.insurance_pnl_ratio_as_f64()?;

        Some(balance > trigger_threshold && pnl_ratio < insurance_pnl_ratio)
    }

    /// Checks if insurance pool equity ADL should be triggered.
    /// According to the API documentation, ADL is triggered when:
    /// `balance` ≤ 0
    pub fn is_insurance_pool_equity_adl_triggered(&self) -> Option<bool> {
        let balance = self.balance_as_f64()?;
        Some(balance <= 0.0)
    }

    /// Checks if contract PnL drawdown ADL should be stopped.
    /// According to the API documentation, ADL stops when:
    /// `pnl_ratio` > `adl_stop_ratio`
    pub fn is_contract_pnl_drawdown_adl_stopped(&self) -> Option<bool> {
        let pnl_ratio = self.pnl_ratio_as_f64()?;
        let adl_stop_ratio = self.adl_stop_ratio_as_f64()?;
        Some(pnl_ratio > adl_stop_ratio)
    }

    /// Checks if insurance pool equity ADL should be stopped.
    /// According to the API documentation, ADL stops when:
    /// `balance` > 0
    pub fn is_insurance_pool_equity_adl_stopped(&self) -> Option<bool> {
        let balance = self.balance_as_f64()?;
        Some(balance > 0.0)
    }

    /// Returns the ADL status for this item.
    /// Returns a tuple of (is_triggered, is_stopped) for both ADL types.
    pub fn adl_status(&self) -> (Option<bool>, Option<bool>, Option<bool>, Option<bool>) {
        (
            self.is_contract_pnl_drawdown_adl_triggered(),
            self.is_contract_pnl_drawdown_adl_stopped(),
            self.is_insurance_pool_equity_adl_triggered(),
            self.is_insurance_pool_equity_adl_stopped(),
        )
    }

    /// Returns true if any ADL condition is currently triggered.
    pub fn is_any_adl_triggered(&self) -> Option<bool> {
        let contract_triggered = self.is_contract_pnl_drawdown_adl_triggered()?;
        let equity_triggered = self.is_insurance_pool_equity_adl_triggered()?;
        Some(contract_triggered || equity_triggered)
    }

    /// Returns true if all ADL conditions are stopped.
    pub fn is_all_adl_stopped(&self) -> Option<bool> {
        let contract_stopped = self.is_contract_pnl_drawdown_adl_stopped()?;
        let equity_stopped = self.is_insurance_pool_equity_adl_stopped()?;
        Some(contract_stopped && equity_stopped)
    }
}

/// Represents the response from the ADL Alert endpoint.
/// Contains the latest update time and a list of ADL alert items.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ADLAlertSummary {
    /// The latest data update timestamp in milliseconds.
    /// Data update frequency is every 1 minute according to the API documentation.
    #[serde(rename = "updatedTime")]
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,

    /// List of ADL alert items.
    /// Contains ADL alert information for various trading pairs.
    pub list: Vec<ADLAlertItem>,
}

impl ADLAlertSummary {
    /// Returns the number of ADL alert items.
    pub fn count(&self) -> usize {
        self.list.len()
    }

    /// Returns true if there are no ADL alert items.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Returns the ADL alert item for a specific symbol, if found.
    pub fn find_by_symbol(&self, symbol: &str) -> Option<&ADLAlertItem> {
        self.list.iter().find(|item| item.symbol == symbol)
    }

    /// Returns all ADL alert items for a specific coin, if any.
    pub fn filter_by_coin(&self, coin: &str) -> Vec<&ADLAlertItem> {
        self.list.iter().filter(|item| item.coin == coin).collect()
    }

    /// Returns all ADL alert items where any ADL condition is triggered.
    pub fn triggered_items(&self) -> Vec<&ADLAlertItem> {
        self.list
            .iter()
            .filter(|item| item.is_any_adl_triggered().unwrap_or(false))
            .collect()
    }

    /// Returns all ADL alert items where all ADL conditions are stopped.
    pub fn stopped_items(&self) -> Vec<&ADLAlertItem> {
        self.list
            .iter()
            .filter(|item| item.is_all_adl_stopped().unwrap_or(false))
            .collect()
    }

    /// Returns the updated time as a chrono DateTime.
    pub fn updated_datetime(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        chrono::DateTime::from_timestamp((self.updated_time / 1000) as i64, 0)
    }

    /// Returns the time since the last update in seconds.
    pub fn time_since_update(&self) -> Option<u64> {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        if now > self.updated_time {
            Some((now - self.updated_time) / 1000)
        } else {
            Some(0)
        }
    }

    /// Checks if the data is stale (older than 2 minutes).
    /// The API updates data every 1 minute, so data older than 2 minutes
    /// might be considered stale.
    pub fn is_stale(&self) -> Option<bool> {
        self.time_since_update().map(|seconds| seconds > 120)
    }
}
