use super::delivery_record_item::DeliveryRecordItem;
use serde::{Deserialize, Serialize};

/// Delivery record response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliveryRecordResponse {
    /// Product type
    pub category: String,
    /// List of delivery records
    pub list: Vec<DeliveryRecordItem>,
    /// Next page cursor
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}
