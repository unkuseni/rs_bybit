use serde::{Deserialize, Serialize};

/// Sub UID response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubUidResponse {
    /// All sub UIDs under the main UID
    #[serde(rename = "subMemberIds")]
    pub sub_member_ids: Vec<String>,
    /// All sub UIDs that have universal transfer enabled
    #[serde(rename = "transferableSubMemberIds")]
    pub transferable_sub_member_ids: Vec<String>,
}
