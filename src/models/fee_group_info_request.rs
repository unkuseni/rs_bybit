use crate::prelude::*;

/// Parameters for requesting fee group information.
///
/// This struct defines the parameters for querying fee group structure and fee rates via Bybit's `/v5/market/fee-group-info` endpoint.
/// The new grouped fee structure only applies to Pro-level and Market Maker clients and does not apply to retail traders.
#[derive(Clone, Default)]
pub struct FeeGroupInfoRequest<'a> {
    /// The product type. Currently only "contract" is supported.
    ///
    /// Specifies the type of instrument for the fee group data. According to the API documentation,
    /// only "contract" is accepted as a value for this parameter.
    pub product_type: Cow<'a, str>,

    /// The group ID. Optional parameter to filter by specific fee group.
    ///
    /// Valid values are: "1", "2", "3", "4", "5", "6", "7", "8".
    /// If not specified, returns information for all fee groups.
    pub group_id: Option<Cow<'a, str>>,
}

impl<'a> FeeGroupInfoRequest<'a> {
    /// Creates a default fee group info request for contracts.
    ///
    /// Returns a `FeeGroupInfoRequest` with `product_type` set to `"contract"` and no group ID filter.
    /// This is the standard request for getting all fee group information for contracts.
    pub fn default() -> FeeGroupInfoRequest<'a> {
        FeeGroupInfoRequest::new("contract", None)
    }

    /// Constructs a new fee group info request with specified parameters.
    ///
    /// # Arguments
    /// * `product_type` - The product type (currently only "contract" is valid)
    /// * `group_id` - Optional group ID to filter by specific fee group
    pub fn new(product_type: &'a str, group_id: Option<&'a str>) -> FeeGroupInfoRequest<'a> {
        FeeGroupInfoRequest {
            product_type: Cow::Borrowed(product_type),
            group_id: group_id.map(Cow::Borrowed),
        }
    }

    /// Constructs a new fee group info request for a specific group ID.
    ///
    /// # Arguments
    /// * `group_id` - The group ID to filter by (e.g., "1", "2", etc.)
    pub fn with_group_id(group_id: &'a str) -> FeeGroupInfoRequest<'a> {
        FeeGroupInfoRequest {
            product_type: Cow::Borrowed("contract"),
            group_id: Some(Cow::Borrowed(group_id)),
        }
    }
}
