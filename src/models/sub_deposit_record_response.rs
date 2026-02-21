use super::deposit_record_item::DepositRecordItem;
use crate::prelude::*;

/// Sub deposit record response (on-chain)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubDepositRecordResponse {
    /// Array of deposit records
    pub rows: Vec<DepositRecordItem>,

    /// Cursor for pagination
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}
