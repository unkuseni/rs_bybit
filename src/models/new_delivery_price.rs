use crate::prelude::*;

/// Represents a new delivery price record for options contracts.
/// This is specifically for the `/v5/market/new-delivery-price` endpoint which returns
/// historical option delivery prices.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewDeliveryPriceItem {
    /// The delivery price at settlement.
    /// This is the price at which the option contract was settled.
    pub delivery_price: String,

    /// The delivery timestamp in milliseconds.
    /// This is when the option contract was settled.
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,
}

impl NewDeliveryPriceItem {
    /// Constructs a new NewDeliveryPriceItem with specified delivery price and time.
    pub fn new(delivery_price: &str, delivery_time: u64) -> Self {
        Self {
            delivery_price: delivery_price.to_string(),
            delivery_time,
        }
    }

    /// Returns the delivery price as a floating-point number.
    /// Returns `None` if the price cannot be parsed.
    pub fn delivery_price_as_f64(&self) -> Option<f64> {
        self.delivery_price.parse::<f64>().ok()
    }

    /// Returns the delivery time as a chrono DateTime.
    pub fn delivery_datetime(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        chrono::DateTime::from_timestamp((self.delivery_time / 1000) as i64, 0)
    }
}

/// Represents the response from the new delivery price endpoint for options.
/// This endpoint returns historical option delivery prices, with the most recent
/// 50 records returned in reverse order of "deliveryTime" by default.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewDeliveryPriceSummary {
    /// The product category (always "option" for this endpoint).
    /// This endpoint is specifically for options contracts.
    pub category: String,

    /// List of new delivery price records.
    /// Contains historical delivery prices for options contracts, sorted by
    /// delivery time in descending order (most recent first).
    pub list: Vec<NewDeliveryPriceItem>,
}

impl NewDeliveryPriceSummary {
    /// Returns the most recent delivery price item, if available.
    pub fn most_recent(&self) -> Option<&NewDeliveryPriceItem> {
        self.list.first()
    }

    /// Returns the oldest delivery price item, if available.
    pub fn oldest(&self) -> Option<&NewDeliveryPriceItem> {
        self.list.last()
    }

    /// Returns the number of delivery price records.
    pub fn count(&self) -> usize {
        self.list.len()
    }

    /// Returns true if there are no delivery price records.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Returns an iterator over delivery price items sorted by delivery time (ascending).
    pub fn sorted_by_time_asc(&self) -> impl Iterator<Item = &NewDeliveryPriceItem> {
        let mut indices: Vec<usize> = (0..self.list.len()).collect();
        indices.sort_by_key(|&i| self.list[i].delivery_time);
        indices.into_iter().map(move |i| &self.list[i])
    }

    /// Returns an iterator over delivery price items sorted by delivery time (descending).
    pub fn sorted_by_time_desc(&self) -> impl Iterator<Item = &NewDeliveryPriceItem> {
        let mut indices: Vec<usize> = (0..self.list.len()).collect();
        indices.sort_by_key(|&i| std::cmp::Reverse(self.list[i].delivery_time));
        indices.into_iter().map(move |i| &self.list[i])
    }

    /// Returns the delivery price for a specific timestamp, if found.
    /// Uses binary search since the list is sorted by delivery time in descending order.
    pub fn find_by_timestamp(&self, timestamp: u64) -> Option<&NewDeliveryPriceItem> {
        self.list
            .binary_search_by_key(&timestamp, |item| item.delivery_time)
            .ok()
            .map(|index| &self.list[index])
    }

    /// Returns the delivery price closest to a specific timestamp.
    pub fn find_closest_to_timestamp(&self, timestamp: u64) -> Option<&NewDeliveryPriceItem> {
        if self.list.is_empty() {
            return None;
        }

        let mut closest = &self.list[0];
        let mut min_diff = timestamp.abs_diff(closest.delivery_time);

        for item in &self.list[1..] {
            let diff = timestamp.abs_diff(item.delivery_time);
            if diff < min_diff {
                min_diff = diff;
                closest = item;
            }
        }

        Some(closest)
    }
}
