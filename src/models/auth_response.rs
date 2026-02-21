use crate::prelude::*;

/// Enum representing WebSocket authentication responses.
///
/// Encapsulates authentication responses for different WebSocket connection types
/// (private streams and trade streams). Bots use this to verify authentication
/// success and handle authentication failures appropriately.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AuthResponse {
    /// Authentication response for private WebSocket streams.
    ///
    /// Used for private data streams (position, execution, order, wallet updates).
    /// Bots should check the `success` field to confirm authentication.
    PrivateAuth(PrivateAuthData),

    /// Authentication response for trade WebSocket streams.
    ///
    /// Used for trade/order entry streams. Has a different format than private streams.
    /// Bots should check the `ret_code` field (0 for success).
    TradeAuth(TradeAuthData),
}

/// Authentication data for private WebSocket streams.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateAuthData {
    /// Whether authentication was successful.
    ///
    /// `true` if authentication succeeded, `false` otherwise.
    /// Bots should check this field before proceeding with private stream operations.
    pub success: bool,

    /// Return message from authentication.
    ///
    /// Typically empty string for success, error message for failure.
    /// Bots should log this for debugging authentication issues.
    #[serde(rename = "ret_msg")]
    pub ret_msg: String,

    /// Operation type, always "auth" for authentication responses.
    ///
    /// Bots use this to identify the message type.
    pub op: String,

    /// Connection ID for the authenticated WebSocket connection.
    ///
    /// A unique identifier for the WebSocket connection.
    /// Bots can use this to track specific connections.
    #[serde(rename = "conn_id")]
    pub conn_id: String,

    /// Request ID (optional).
    ///
    /// The ID of the authentication request, if provided.
    /// Bots can use this to correlate requests and responses.
    #[serde(rename = "req_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
}

/// Authentication data for trade WebSocket streams.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeAuthData {
    /// Return code from authentication.
    ///
    /// `0` for success, non-zero for errors.
    /// Common error codes:
    /// - `10004`: invalid signature
    /// - `10001`: parameter error
    /// - `20001`: repeat authentication
    /// Bots should check this field before proceeding with trade operations.
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message from authentication.
    ///
    /// "OK" for success, error message for failure.
    /// Bots should log this for debugging authentication issues.
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Operation type, always "auth" for authentication responses.
    ///
    /// Bots use this to identify the message type.
    pub op: String,

    /// Connection ID for the authenticated WebSocket connection.
    ///
    /// A unique identifier for the WebSocket connection.
    /// Bots can use this to track specific connections.
    #[serde(rename = "connId")]
    pub conn_id: String,

    /// Request ID (optional).
    ///
    /// The ID of the authentication request, if provided.
    /// Bots can use this to correlate requests and responses.
    #[serde(rename = "reqId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
}

impl PrivateAuthData {
    /// Creates a new PrivateAuthData instance.
    pub fn new(success: bool, ret_msg: &str, conn_id: &str, req_id: Option<&str>) -> Self {
        Self {
            success,
            ret_msg: ret_msg.to_string(),
            op: "auth".to_string(),
            conn_id: conn_id.to_string(),
            req_id: req_id.map(|s| s.to_string()),
        }
    }

    /// Returns true if authentication was successful.
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Returns true if authentication failed.
    pub fn is_failure(&self) -> bool {
        !self.success
    }
}

impl TradeAuthData {
    /// Creates a new TradeAuthData instance.
    pub fn new(ret_code: i32, ret_msg: &str, conn_id: &str, req_id: Option<&str>) -> Self {
        Self {
            ret_code,
            ret_msg: ret_msg.to_string(),
            op: "auth".to_string(),
            conn_id: conn_id.to_string(),
            req_id: req_id.map(|s| s.to_string()),
        }
    }

    /// Returns true if authentication was successful (ret_code == 0).
    pub fn is_success(&self) -> bool {
        self.ret_code == 0
    }

    /// Returns true if authentication failed (ret_code != 0).
    pub fn is_failure(&self) -> bool {
        self.ret_code != 0
    }

    /// Returns the authentication error code if authentication failed.
    pub fn error_code(&self) -> Option<i32> {
        if self.is_failure() {
            Some(self.ret_code)
        } else {
            None
        }
    }
}

impl AuthResponse {
    /// Returns true if authentication was successful.
    pub fn is_success(&self) -> bool {
        match self {
            AuthResponse::PrivateAuth(data) => data.is_success(),
            AuthResponse::TradeAuth(data) => data.is_success(),
        }
    }

    /// Returns true if authentication failed.
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    /// Returns the connection ID from the authentication response.
    pub fn conn_id(&self) -> &str {
        match self {
            AuthResponse::PrivateAuth(data) => &data.conn_id,
            AuthResponse::TradeAuth(data) => &data.conn_id,
        }
    }

    /// Returns the return message from the authentication response.
    pub fn ret_msg(&self) -> &str {
        match self {
            AuthResponse::PrivateAuth(data) => &data.ret_msg,
            AuthResponse::TradeAuth(data) => &data.ret_msg,
        }
    }

    /// Returns the error code if authentication failed.
    pub fn error_code(&self) -> Option<i32> {
        match self {
            AuthResponse::PrivateAuth(data) => {
                if data.is_failure() {
                    // For private auth, we don't have a numeric error code
                    // Return -1 to indicate failure without specific code
                    Some(-1)
                } else {
                    None
                }
            }
            AuthResponse::TradeAuth(data) => data.error_code(),
        }
    }
}
