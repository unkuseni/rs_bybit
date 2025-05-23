use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BybitApiResponse<T = Value, U = Empty, V = u64> {
    /// The return code indicating the success or failure of the request.
    ///
    /// A value of `0` indicates success, while non-zero values correspond to specific error codes as defined in the Bybit API documentation (e.g., `10001` for invalid parameters). Trading bots should check this field to handle errors gracefully, such as retrying requests or logging issues for debugging.
    pub ret_code: i32,

    /// A human-readable message describing the result or error.
    ///
    /// For successful requests, this is typically `"OK"`. For errors, it provides a description of the issue (e.g., `"invalid timestamp"`). Bots should log this field for debugging and user feedback, especially when `ret_code` is non-zero.
    pub ret_msg: String,

    /// Contains the actual payload corresponding to the requested endpoint, as JSON.
    pub result: T,

    /// Additional information, typically an empty object.
    ///
    /// This field is usually an empty `Empty` struct (`{}`) and can be ignored in most cases. However, bots should verify its presence to ensure response consistency.
    #[serde(default)]
    pub ret_ext_info: U,

    /// The timestamp of the response in milliseconds.
    ///
    /// This represents the time the response was generated by Bybit's server. Trading bots can use this to measure latency or further synchronize operations, though `result.time_second` is typically more relevant for timestamp validation.
    #[serde(default)]
    pub time: V,
}
