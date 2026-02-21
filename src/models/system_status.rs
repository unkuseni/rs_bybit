use crate::prelude::*;

/// Represents a single system status/maintenance record
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatusItem {
    /// Unique identifier for the system status record
    pub id: String,

    /// Title of the system maintenance
    pub title: String,

    /// System state (e.g., "completed", "in_progress", "scheduled")
    pub state: String,

    /// Start time of system maintenance, timestamp in milliseconds
    #[serde(with = "string_to_u64")]
    pub begin: u64,

    /// End time of system maintenance, timestamp in milliseconds.
    /// Before maintenance is completed, it is the expected end time;
    /// After maintenance is completed, it will be changed to the actual end time.
    #[serde(with = "string_to_u64")]
    pub end: u64,

    /// Hyperlink to system maintenance details. Default value is empty string
    pub href: String,

    /// Service types affected by the maintenance
    pub service_types: Vec<u32>,

    /// Products affected by the maintenance
    pub product: Vec<u32>,

    /// Affected UID tail numbers
    pub uid_suffix: Vec<u32>,

    /// Maintenance type
    #[serde(with = "string_to_u32")]
    pub maintain_type: u32,

    /// Environment
    #[serde(with = "string_to_u32")]
    pub env: u32,
}

/// Represents the response from the system status endpoint
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatusResult {
    /// List of system status/maintenance records
    pub list: Vec<SystemStatusItem>,
}
