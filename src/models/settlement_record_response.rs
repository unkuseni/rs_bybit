use super::settlement_record_item::SettlementRecordItem;
use serde::{Deserialize, Serialize};

/// Settlement record response for USDC perpetual and futures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettlementRecordResponse {
    /// Product type
    pub category: String,
    /// List of settlement records
    pub list: Vec<SettlementRecordItem>,
    /// Next page cursor
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}
