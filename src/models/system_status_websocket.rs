use crate::prelude::*;

/// Represents a single system status/maintenance record in WebSocket stream
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatusWebsocketItem {
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
    #[serde(rename = "serviceTypes")]
    pub service_types: Vec<u32>,

    /// Products affected by the maintenance
    pub product: Vec<u32>,

    /// Affected UID tail numbers
    #[serde(rename = "uidSuffix")]
    pub uid_suffix: Vec<u32>,

    /// Maintenance type
    #[serde(rename = "maintainType")]
    #[serde(with = "string_to_u32")]
    pub maintain_type: u32,

    /// Environment
    #[serde(with = "string_to_u32")]
    pub env: u32,
}

impl SystemStatusWebsocketItem {
    /// Constructs a new SystemStatusWebsocketItem with specified parameters
    pub fn new(
        id: &str,
        title: &str,
        state: &str,
        begin: u64,
        end: u64,
        href: &str,
        service_types: Vec<u32>,
        product: Vec<u32>,
        uid_suffix: Vec<u32>,
        maintain_type: u32,
        env: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            state: state.to_string(),
            begin,
            end,
            href: href.to_string(),
            service_types,
            product,
            uid_suffix,
            maintain_type,
            env,
        }
    }

    /// Returns true if the maintenance is currently in progress
    pub fn is_in_progress(&self) -> bool {
        self.state.to_lowercase() == "in_progress"
    }

    /// Returns true if the maintenance is scheduled for the future
    pub fn is_scheduled(&self) -> bool {
        self.state.to_lowercase() == "scheduled"
    }

    /// Returns true if the maintenance has been completed
    pub fn is_completed(&self) -> bool {
        self.state.to_lowercase() == "completed"
    }

    /// Returns true if the maintenance affects the given service type
    pub fn affects_service_type(&self, service_type: u32) -> bool {
        self.service_types.contains(&service_type)
    }

    /// Returns true if the maintenance affects the given product
    pub fn affects_product(&self, product_id: u32) -> bool {
        self.product.contains(&product_id)
    }

    /// Returns true if the maintenance affects the given UID suffix
    pub fn affects_uid_suffix(&self, uid_suffix: u32) -> bool {
        self.uid_suffix.contains(&uid_suffix)
    }

    /// Returns true if the maintenance is currently active (in progress)
    pub fn is_active(&self, current_time: u64) -> bool {
        self.is_in_progress() && current_time >= self.begin && current_time <= self.end
    }

    /// Returns the duration of the maintenance in milliseconds
    pub fn duration(&self) -> u64 {
        if self.end > self.begin {
            self.end - self.begin
        } else {
            0
        }
    }
}

/// Represents a WebSocket system status update event
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemStatusUpdate {
    /// The WebSocket topic for the event (e.g., "system.status")
    ///
    /// Specifies the data stream for system status updates.
    #[serde(rename = "topic")]
    pub topic: String,

    /// The timestamp when the system generated the data (ms)
    #[serde(rename = "ts")]
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,

    /// The event data containing system status records
    #[serde(rename = "data")]
    pub data: Vec<SystemStatusWebsocketItem>,
}

impl SystemStatusUpdate {
    /// Constructs a new SystemStatusUpdate
    pub fn new(topic: &str, timestamp: u64, data: Vec<SystemStatusWebsocketItem>) -> Self {
        Self {
            topic: topic.to_string(),
            timestamp,
            data,
        }
    }

    /// Returns the first in-progress maintenance record, if any
    pub fn first_in_progress(&self) -> Option<&SystemStatusWebsocketItem> {
        self.data.iter().find(|item| item.is_in_progress())
    }

    /// Returns all scheduled maintenance records
    pub fn scheduled_items(&self) -> Vec<&SystemStatusWebsocketItem> {
        self.data
            .iter()
            .filter(|item| item.is_scheduled())
            .collect()
    }

    /// Returns all completed maintenance records
    pub fn completed_items(&self) -> Vec<&SystemStatusWebsocketItem> {
        self.data
            .iter()
            .filter(|item| item.is_completed())
            .collect()
    }

    /// Returns all in-progress maintenance records
    pub fn in_progress_items(&self) -> Vec<&SystemStatusWebsocketItem> {
        self.data
            .iter()
            .filter(|item| item.is_in_progress())
            .collect()
    }

    /// Returns true if there are any active maintenance events
    pub fn has_active_maintenance(&self, current_time: u64) -> bool {
        self.data.iter().any(|item| item.is_active(current_time))
    }

    /// Returns maintenance items affecting a specific service type
    pub fn items_by_service_type(&self, service_type: u32) -> Vec<&SystemStatusWebsocketItem> {
        self.data
            .iter()
            .filter(|item| item.affects_service_type(service_type))
            .collect()
    }

    /// Returns maintenance items affecting a specific product
    pub fn items_by_product(&self, product_id: u32) -> Vec<&SystemStatusWebsocketItem> {
        self.data
            .iter()
            .filter(|item| item.affects_product(product_id))
            .collect()
    }

    /// Returns the maintenance item with the given ID, if it exists
    pub fn find_by_id(&self, id: &str) -> Option<&SystemStatusWebsocketItem> {
        self.data.iter().find(|item| item.id == id)
    }
}
