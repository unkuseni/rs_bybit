use crate::prelude::*;

/// Response for querying available WASPs (Virtual Asset Service Providers)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaspListResponse {
    /// List of exchange entity info
    pub vasp: Vec<VaspInfo>,
}

/// VASP (Virtual Asset Service Provider) information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaspInfo {
    /// Receiver platform id.
    /// When transfer to the exchanges that are not in the list, please use vaspEntityId='others'
    #[serde(rename = "vaspEntityId")]
    pub vasp_entity_id: String,

    /// Receiver platform name
    #[serde(rename = "vaspName")]
    pub vasp_name: String,
}
