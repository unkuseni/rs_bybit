use super::internal_deposit_record_item::InternalDepositRecordItem;
use crate::prelude::*;

/// Internal deposit record response (off-chain)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalDepositRecordResponse {
    /// Array of internal deposit records
    pub rows: Vec<InternalDepositRecordItem>,

    /// Cursor for pagination
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}
