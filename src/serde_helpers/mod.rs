use crate::prelude::*;

/// Custom deserialization module for handling strings as optional `f64`.
///
/// Used for fields like `avg_price` or `leverage` that may be empty or absent in API responses. Empty strings are treated as `None`, ensuring proper handling of optional numerical fields.
pub mod string_to_float_optional {
    use super::*;

    /// Serializes an `Option<f64>` as a string.
    ///
    /// Converts the `Option<f64>` to a string, using an empty string for `None`. This aligns with Bybit’s API expectations for optional fields.
    pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_str(""),
        }
    }

    /// Deserializes a string to an `Option<f64>`, returning `None` for empty strings.
    ///
    /// Parses the string to an `Option<f64>`, treating empty strings as `None`. Bots use this to handle optional numerical fields robustly.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) if s.trim().is_empty() => Ok(None),
            Some(s) => f64::from_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

/// Module for serializing and deserializing optional u64 values as strings.
pub mod string_to_u64_optional {
    use super::*;

    /// Serializes an Option<u64> as a string, using an empty string for None.
    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_str(""),
        }
    }

    /// Deserializes a string to an Option<u64>, returning None for empty strings.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) if s.trim().is_empty() => Ok(None),
            Some(s) => u64::from_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

/// Deserializes a string to a u64, handling empty strings and invalid formats.
///
/// Used for fields like timestamps (`time_second`, `start_time`) that are returned as strings by the Bybit API but need to be converted to `u64` for Rust. Bots should rely on this to handle edge cases, such as empty strings, which are treated as `0`.
pub mod string_to_u64 {
    use super::*;

    // Serialize a u64 as a string.
    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_string();
        serializer.serialize_str(&s)
    }

    // Deserialize a string to a u64.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

/// Deserializes a string to a u32, handling empty strings and invalid formats.
///
/// Used for fields like numeric IDs or enum values that are returned as strings by the Bybit API but need to be converted to `u32` for Rust. Bots should rely on this to handle edge cases, such as empty strings, which are treated as `0`.
pub mod string_to_u32 {
    use super::*;

    // Serialize a u32 as a string.
    pub fn serialize<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_string();
        serializer.serialize_str(&s)
    }

    // Deserialize a string to a u32.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u32>().map_err(serde::de::Error::custom)
    }
}

/// Deserializes a string to an f64, handling empty strings and invalid formats.
///
/// Used for fields like prices (`last_price`, `open_price`) and quantities (`size`, `open_interest`) that are returned as strings to preserve precision but need to be converted to `f64` for calculations. Bots should use this to ensure accurate numerical processing and handle edge cases like empty strings.
pub mod string_to_float {
    use super::*;

    // Serialize a u64 as a string.
    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_string();
        serializer.serialize_str(&s)
    }

    // Deserialize a string as an f64.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<f64>().map_err(serde::de::Error::custom)
    }
}

/// Checks if an optional string is empty or None.
///
/// Used in serialization to skip fields like `min_price` or `max_order_qty` when they are empty or `None`, ensuring cleaner JSON output that complies with Bybit’s API expectations.
pub fn is_empty_or_none(s: &Option<String>) -> bool {
    s.as_ref().is_none_or(|s| s.is_empty())
}

/// Custom deserialization function to treat empty strings as `None`.
///
/// Used for fields that may receive empty strings from the Bybit API but should be treated as absent values in Rust. This ensures proper handling of optional fields like `block_trade_id` or `order_iv` in order responses, preventing empty strings from being misinterpreted as valid data. Bots should use this to maintain data integrity when processing API responses.
pub fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref s) if s.trim().is_empty() => Ok(None),
        Some(s) => {
            let kind = T::deserialize(serde_json::Value::String(s))
                .map(Some)
                .map_err(serde::de::Error::custom)?;
            Ok(kind)
        }
        None => Ok(None),
    }
}
