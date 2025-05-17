use log::error;
use serde_json::{json, Value};

use crate::api::{Trade, API};
use crate::client::Client;
use crate::errors::BybitError;
use crate::model::{
    AmendOrderRequest, AmendOrderResponse, BatchAmendRequest, BatchAmendResponse,
    BatchCancelRequest, BatchCancelResponse, BatchPlaceRequest, BatchPlaceResponse,
    CancelOrderRequest, CancelOrderResponse, CancelallRequest, CancelallResponse, Category,
    OpenOrdersRequest, OpenOrdersResponse, OrderHistoryRequest, OrderHistoryResponse, OrderRequest,
    OrderResponse, OrderType, RequestType, Side, TradeHistoryRequest, TradeHistoryResponse,
};
use crate::util::{build_json_request, build_request, date_to_milliseconds, generate_random_uid};

use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Trader {
    pub client: Client,
    pub recv_window: u16,
}

/// Creates an order with various options for different account types and contract types.
///
/// # Account Coverage
/// - Unified account: Spot, USDT perpetual, USDC contract, Inverse contract, Option
/// - Classic account: Spot, USDT perpetual, Inverse contract
///
/// # Order Types
/// - Limit Order: Must specify quantity and price.
/// - Market Order: Executes at the best market price. Price parameter can be empty.
/// - Conditional Order: Set `triggerPrice` to convert to a conditional order.
///
/// # Time In Force Strategies
/// - GTC (Good Till Cancelled)
/// - IOC (Immediate Or Cancel)
/// - FOK (Fill Or Kill)
/// - PostOnly: Cancelled if it would be filled immediately when submitted.
///
/// # Take Profit / Stop Loss
/// - Can be set during order placement and modified later.
///
/// # Order Quantity
/// - Only positive numbers are supported for perpetual contract orders.
///
/// # Order Price
/// - Required for limit orders. Must be higher than liquidation price if in position.
///
/// # Order Link ID
/// - Custom active order ID up to 36 characters.
///
/// # Order Limits
/// - Futures: 500 active orders per contract, 10 conditional orders per account.
/// - Spot: 500 total orders, 30 open TP/SL orders, 30 open conditional orders.
/// - Option: 50 open orders.
///
/// # Rate Limit
/// - Refer to the rate limit table. Contact client manager for increases.
///
/// # Risk Control
/// - Bybit monitors API requests. Exceeding daily limits may lead to restrictions.
///
/// # Spot Stop Order in Different Account Types
/// - Classic account: New order ID upon stop order trigger.
/// - Unified account: Order ID remains unchanged upon stop order trigger.
pub enum Action<'a> {
    Order(OrderRequest<'a>, bool),
    Amend(AmendOrderRequest<'a>, bool),
    Cancel(CancelOrderRequest<'a>, bool),
}

impl Trader {
    pub async fn place_custom_order<'b>(
        &self,
        req: OrderRequest<'_>,
    ) -> Result<OrderResponse, BybitError> {
        let action = Action::Order(req, false);
        let parameters = Self::build_orders(action);

        let request = build_json_request(&parameters);
        let response: OrderResponse = self
            .client
            .post_signed(
                API::Trade(Trade::Place),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn place_futures_limit_order(
        &self,
        category: Category,
        symbol: &str,
        side: Side,
        qty: f64,
        price: f64,
        mode: u8,
    ) -> Result<OrderResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let req = OrderRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            side,
            qty,
            order_type: OrderType::Limit,
            position_idx: Some(mode),
            order_link_id: Some(generate_random_uid(36).into()),
            price: Some(price),
            ..Default::default()
        };
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into_owned());
        parameters.insert("orderType".into(), req.order_type.as_str().into());
        parameters.insert("side".into(), req.side.as_str().into());
        if let Some(v) = req.order_link_id {
            parameters.insert("orderLinkId".into(), v.into());
        }
        parameters.insert("qty".into(), req.qty.to_string());
        if let Some(v) = req.position_idx {
            match v {
                0 | 1 | 2 => {
                    parameters.insert("positionIdx".into(), v.to_string());
                }
                _ => return Err(BybitError::from("Invalid position index".to_string())),
            }
        }
        if let Some(v) = req.price {
            parameters.insert("price".into(), v.to_string());
        }
        parameters.insert("timeInForce".into(), "GTC".into());
        let request = build_json_request(&parameters);
        let response: OrderResponse = self
            .client
            .post_signed(
                API::Trade(Trade::Place),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn amend_order<'b>(
        &self,
        req: AmendOrderRequest<'_>,
    ) -> Result<AmendOrderResponse, BybitError> {
        let action = Action::Amend(req, false);
        let parameters = Self::build_orders(action);
        let request = build_json_request(&parameters);
        let response: AmendOrderResponse = self
            .client
            .post_signed(
                API::Trade(Trade::Amend),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }
    pub async fn cancel_order<'b>(
        &self,
        req: CancelOrderRequest<'_>,
    ) -> Result<CancelOrderResponse, BybitError> {
        let action = Action::Cancel(req, false);
        let parameters = Self::build_orders(action);
        let request = build_json_request(&parameters);
        let response: CancelOrderResponse = self
            .client
            .post_signed(
                API::Trade(Trade::Cancel),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }
    pub async fn get_open_orders<'b>(
        &self,
        req: OpenOrdersRequest<'_>,
    ) -> Result<OpenOrdersResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());

        if let Some(base_coin) = req.base_coin {
            parameters.insert("baseCoin".into(), base_coin.into());
        }
        if let Some(settle_coin) = req.settle_coin {
            parameters.insert("settleCoin".into(), settle_coin.into());
        }
        if let Some(order_id) = req.order_id {
            parameters.insert("orderId".into(), order_id.into());
        }
        if let Some(order_link_id) = req.order_link_id {
            parameters.insert("orderLinkId".into(), order_link_id.into());
        }
        if let Some(open_only) = req.open_only {
            if matches!(open_only, 0 | 1 | 2) {
                parameters.insert("openOnly".into(), open_only.to_string().into());
            }
        }
        if let Some(order_filter) = req.order_filter {
            parameters.insert("orderFilter".into(), order_filter.into());
        }
        if let Some(limit) = req.limit {
            parameters.insert("limit".into(), limit.to_string().into());
        }

        let request = build_request(&parameters);
        let response: OpenOrdersResponse = self
            .client
            .get_signed(API::Trade(Trade::OpenOrders), 5000, Some(request))
            .await?;

        Ok(response)
    }
    pub async fn cancel_all_orders<'b>(
        &self,
        req: CancelallRequest<'_>,
    ) -> Result<CancelallResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        if let Some(base_coin) = req.base_coin {
            parameters.insert("baseCoin".into(), base_coin.into());
        }
        if let Some(settle_coin) = req.settle_coin {
            parameters.insert("settleCoin".into(), settle_coin.into());
        }
        if let Some(order_filter) = req.order_filter {
            parameters.insert("orderFilter".into(), order_filter.into());
        }
        if let Some(stop_order_type) = req.stop_order_type {
            parameters.insert("stopOrderType".into(), stop_order_type.into());
        }
        let request = build_json_request(&parameters);
        let response: CancelallResponse = self
            .client
            .post_signed(
                API::Trade(Trade::CancelAll),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Retrieves the order history based on the given request parameters.
    ///
    /// # Arguments
    /// * `req` - An instance of `OrderHistoryRequest` containing the request parameters.
    ///
    /// # Returns
    /// A `Result` wrapping `OrderHistory` which contains the historical orders' data.
    /// If the operation fails, it returns an error.
    ///
    pub async fn get_order_history<'b>(
        &self,
        req: OrderHistoryRequest<'_>,
    ) -> Result<OrderHistoryResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        req.symbol
            .map(|symbol| parameters.insert("symbol".into(), symbol.into()));
        req.base_coin
            .map(|base_coin| parameters.insert("baseCoin".into(), base_coin.into()));
        req.settle_coin
            .map(|settle_coin| parameters.insert("settleCoin".into(), settle_coin.into()));
        req.order_id
            .map(|order_id| parameters.insert("orderId".into(), order_id.into()));
        req.order_link_id
            .map(|order_link_id| parameters.insert("orderLinkId".into(), order_link_id.into()));
        req.order_filter
            .map(|order_filter| parameters.insert("orderFilter".into(), order_filter.into()));
        req.order_status
            .map(|order_status| parameters.insert("orderStatus".into(), order_status.into()));
        req.start_time
            .map(|start_time| parameters.insert("startTime".into(), start_time.to_string()));
        req.end_time
            .map(|end_time| parameters.insert("endTime".into(), end_time.to_string()));
        req.limit
            .map(|limit| parameters.insert("limit".into(), limit.to_string()));

        let request = build_request(&parameters);
        let response: OrderHistoryResponse = self
            .client
            .get_signed(
                API::Trade(Trade::History),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Retrieves the trade history for a specific trading pair, order, or time range.
    ///
    /// # Arguments
    ///
    /// * `req` - A `TradeHistoryRequest` containing the parameters for the trade history query.
    ///
    /// # Returns
    ///
    /// Returns a `Result<TradeHistoryResponse, BybitError>` containing the trade history data if the query is successful, or an error detailing the problem if the query fails.
    pub async fn get_trade_history<'b>(
        &self,
        req: TradeHistoryRequest<'_>,
    ) -> Result<TradeHistoryResponse, BybitError> {
        // Create a new BTreeMap to store the parameters for the request
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        // Add the category to the request parameters
        parameters.insert("category".into(), req.category.as_str().into());

        // Add the symbol to the request parameters if it is specified
        req.symbol
            .map(|symbol| parameters.insert("symbol".into(), symbol.into()));

        // Add the order ID to the request parameters if it is specified
        req.order_id
            .map(|order_id| parameters.insert("orderId".into(), order_id.into()));

        // Add the order link ID to the request parameters if it is specified
        req.order_link_id
            .map(|order_link_id| parameters.insert("orderLinkId".into(), order_link_id.into()));

        // Add the base coin to the request parameters if it is specified
        req.base_coin
            .map(|base_coin| parameters.insert("baseCoin".into(), base_coin.into()));

        // Add the start time to the request parameters if it is specified
        req.start_time
            .map(|start_time| parameters.insert("startTime".into(), start_time.to_string()));

        // Add the end time to the request parameters if it is specified
        req.end_time
            .map(|end_time| parameters.insert("endTime".into(), end_time.to_string()));

        // Add the limit to the request parameters if it is specified
        req.limit
            .map(|limit| parameters.insert("limit".into(), limit.to_string()));

        // Add the execution type to the request parameters if it is specified
        req.exec_type
            .map(|exec_type| parameters.insert("execType".into(), exec_type.into()));

        // Build the request from the parameters
        let request = build_request(&parameters);

        // Send the signed GET request to the Bybit API to retrieve the trade history
        let response: TradeHistoryResponse = self
            .client
            .get_signed(
                API::Trade(Trade::TradeHistory),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        // Return the response
        Ok(response)
    }

    /// Asynchronously places a batch of orders using the Bybit API.
    ///
    /// # Arguments
    ///
    /// * `req` - The `BatchPlaceRequest` containing the orders to be placed.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `BatchPlaceResponse` on success or a `BybitError` on failure.
    pub async fn batch_place_order<'b>(
        &self,
        req: BatchPlaceRequest<'_>,
    ) -> Result<BatchPlaceResponse, BybitError> {
        // Create a new BTreeMap to store the parameters for the request
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // Check if the category is valid and insert it into the parameters
        match req.category {
            Category::Linear | Category::Inverse | Category::Option => {
                parameters.insert("category".into(), req.category.as_str().into());
            }
            // If the category is invalid, print an error message
            _ => {
                error!("Invalid category");
            }
        }

        // Create an empty array to store the orders
        let mut requests_array: Vec<Value> = Vec::new();

        // Iterate over each order in the requests and build the orders
        for value in req.requests {
            // Create an Action to represent the order
            let action = Action::Order(value, true);

            // Build the orders using the build_orders method
            let order_object = Self::build_orders(action);

            // Convert the order object to a JSON Value
            let built_orders = json!(order_object);

            // Add the built orders to the requests array
            requests_array.push(built_orders);
        }

        // Insert the requests array into the parameters
        parameters.insert("request".into(), Value::Array(requests_array));

        // Build the request from the parameters
        let request = build_json_request(&parameters);

        // Send the signed POST request to the Bybit API to place the batch of orders
        let response: BatchPlaceResponse = self
            .client
            .post_signed(
                API::Trade(Trade::BatchPlace),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        // Return the response
        Ok(response)
    }

    /// Sends a batch request to amend multiple orders at once.
    ///
    /// # Arguments
    ///
    /// * `req` - The `BatchAmendRequest` containing the orders to amend.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `BatchAmendResponse` on success or a `BybitError` on failure.
    pub async fn batch_amend_order<'b>(
        &self,
        req: BatchAmendRequest<'_>,
    ) -> Result<BatchAmendResponse, BybitError> {
        // Create an empty map to store the parameters
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // Insert the category into the parameters
        match req.category {
            Category::Linear | Category::Inverse | Category::Option => {
                parameters.insert("category".into(), req.category.as_str().into());
            }
            _ => {
                // Print an error message if the category is invalid
                error!("Invalid category");
            }
        }

        // Create an empty array to store the requests
        let mut requests_array: Vec<Value> = Vec::new();

        // Iterate over each request in the BatchAmendRequest
        for value in req.requests {
            // Create an Action to represent the request
            let action = Action::Amend(value, true);

            // Build the orders using the build_orders method
            let amend_object = Self::build_orders(action); // Assuming this returns the correct object structure

            // Convert the amend object to a JSON Value
            let built_amends = json!(amend_object);

            // Add the built amends to the requests array
            requests_array.push(built_amends);
        }

        // Insert the requests array into the parameters
        parameters.insert("request".into(), Value::Array(requests_array));

        // Build the request from the parameters
        let request = build_json_request(&parameters);

        // Send the signed POST request to the Bybit API to amend the batch of orders
        let response: BatchAmendResponse = self
            .client
            .post_signed(
                API::Trade(Trade::BatchAmend),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        // Return the response
        Ok(response)
    }

    /// Cancel a batch of orders from the Bybit API
    ///
    /// This function will send a signed POST request to the Bybit API to cancel
    /// a batch of orders. The request should contain the category, symbol, order_id,
    /// and order_link_id for each order to be cancelled.
    ///
    /// # Arguments
    ///
    /// * `req` - A `BatchCancelRequest` containing the details of the orders to
    /// be cancelled.
    ///
    /// # Returns
    ///
    /// A `BatchCancelResponse` containing the result of the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the Bybit API returns an
    /// error response.
    pub async fn batch_cancel_order<'b>(
        &self,
        req: BatchCancelRequest<'_>,
    ) -> Result<BatchCancelResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        match req.category {
            Category::Linear | Category::Inverse | Category::Option => {
                parameters.insert("category".into(), req.category.as_str().into());
            }
            _ => {
                error!("Invalid category");
            }
        }
        let mut requests_array: Vec<Value> = Vec::new();
        for value in req.requests {
            let action = Action::Cancel(value, true);
            let cancel_object = Self::build_orders(action); // Assuming this returns the correct object structure
            let built_cancels = json!(cancel_object);
            requests_array.push(built_cancels);
        }
        parameters.insert("request".into(), Value::Array(requests_array));
        let request = build_json_request(&parameters);
        let response: BatchCancelResponse = self
            .client
            .post_signed(
                API::Trade(Trade::BatchCancel),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn get_borrow_quota_spot(&self) {
        // TODO: Implement this function
        todo!("This function has not yet been implemented");
    }

    pub async fn set_dcp_options(&self) {
        // TODO: Implement this function
        todo!("This function has not yet been implemented");
    }

    pub fn build_orders<'b>(action: Action<'_>) -> BTreeMap<String, Value> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        match action {
            Action::Order(req, batch) => {
                if batch == false {
                    parameters.insert("category".into(), req.category.as_str().into());
                }
                parameters.insert("symbol".into(), req.symbol.into());
                if let Some(leverage) = req.is_leverage {
                    if leverage {
                        // Whether to borrow. Valid for Unified spot only. 0(default): false then spot trading, 1: true then margin trading
                        parameters.insert("leverage".into(), 1.into());
                    }
                }
                parameters.insert("side".into(), req.side.as_str().into());
                parameters.insert("orderType".into(), req.order_type.as_str().into());

                parameters.insert("qty".into(), req.qty.to_string().into());
                if let Some(market_unit) = req.market_unit {
                    parameters.insert("marketUnit".into(), market_unit.to_string().into());
                }
                if let Some(price) = req.price {
                    parameters.insert("price".into(), price.to_string().into());
                }
                if let Some(trigger_direction) = req.trigger_direction {
                    if trigger_direction {
                        parameters.insert("triggerDirection".into(), 1.into());
                    } else {
                        parameters.insert("triggerDirection".into(), 2.into());
                    }
                }
                if let Some(order_filter) = req.order_filter {
                    parameters.insert("orderFilter".into(), order_filter.into());
                }
                if let Some(trigger_price) = req.trigger_price {
                    parameters.insert("triggerPrice".into(), trigger_price.to_string().into());
                }
                if let Some(trigger) = req.trigger_by {
                    parameters.insert("triggerBy".into(), trigger.into());
                }
                if let Some(iv) = req.order_iv {
                    parameters.insert("orderIv".into(), iv.to_string().into());
                }
                if let Some(time_in_force) = req.time_in_force {
                    parameters.insert("timeInForce".into(), time_in_force.into());
                }
                if let Some(v) = req.position_idx {
                    match v {
                        0 | 1 | 2 => {
                            parameters.insert("positionIdx".into(), v.to_string().into());
                        }
                        _ => error!("Invalid position idx"),
                    }
                }
                if let Some(order_link_id) = req.order_link_id {
                    parameters.insert("orderLinkId".into(), order_link_id.into());
                } else {
                    let uuid = generate_random_uid(36);
                    parameters.insert("orderLinkId".into(), uuid.into());
                }
                if let Some(price) = req.take_profit {
                    parameters.insert("takeProfit".into(), price.to_string().into());
                }
                if let Some(price) = req.stop_loss {
                    parameters.insert("stopLoss".into(), price.to_string().into());
                }
                if let Some(kind) = req.tp_trigger_by {
                    parameters.insert("tpTriggerBy".into(), kind.into());
                }
                if let Some(kind) = req.sl_trigger_by {
                    parameters.insert("slTriggerBy".into(), kind.into());
                }
                if let Some(reduce) = req.reduce_only {
                    parameters.insert("reduceOnly".into(), reduce.into());
                }
                if let Some(close) = req.close_on_trigger {
                    parameters.insert("closeOnTrigger".into(), close.into());
                }
                if let Some(v) = req.mmp {
                    parameters.insert("mmp".into(), v.into());
                }
                if let Some(v) = req.tpsl_mode {
                    parameters.insert("tpslMode".into(), v.into());
                }
                if let Some(v) = req.tp_limit_price {
                    parameters.insert("tpTriggerPrice".into(), v.to_string().into());
                }
                if let Some(v) = req.sl_limit_price {
                    parameters.insert("slTriggerPrice".into(), v.to_string().into());
                }
                if let Some(v) = req.tp_order_type {
                    parameters.insert("tpOrderType".into(), v.into());
                }
                if let Some(v) = req.sl_order_type {
                    parameters.insert("slOrderType".into(), v.into());
                }
            }
            Action::Amend(req, batch) => {
                if batch == false {
                    parameters.insert("category".into(), req.category.as_str().into());
                }
                parameters.insert("symbol".into(), req.symbol.into());
                if let Some(v) = req.order_id {
                    parameters.insert("orderId".into(), v.into());
                }
                if let Some(v) = req.order_link_id {
                    parameters.insert("orderLinkId".into(), v.into());
                }
                if let Some(v) = req.order_iv {
                    parameters.insert("orderIv".into(), v.to_string().into());
                }
                if let Some(v) = req.trigger_price {
                    parameters.insert("triggerPrice".into(), v.to_string().into());
                }
                parameters.insert("qty".into(), req.qty.into());
                if let Some(v) = req.price {
                    parameters.insert("price".into(), v.to_string().into());
                }
                if let Some(v) = req.tpsl_mode {
                    parameters.insert("tpslMode".into(), v.into());
                }
                if let Some(v) = req.take_profit {
                    parameters.insert("takeProfit".into(), v.to_string().into());
                }
                if let Some(v) = req.stop_loss {
                    parameters.insert("stopLoss".into(), v.to_string().into());
                }
                if let Some(v) = req.tp_trigger_by {
                    parameters.insert("tpTriggerBy".into(), v.into());
                }
                if let Some(v) = req.sl_trigger_by {
                    parameters.insert("slTriggerBy".into(), v.into());
                }
                if let Some(v) = req.trigger_by {
                    parameters.insert("triggerBy".into(), v.into());
                }
                if let Some(v) = req.tp_limit_price {
                    parameters.insert("tpLimitPrice".into(), v.to_string().into());
                }
                if let Some(v) = req.sl_limit_price {
                    parameters.insert("slLimitPrice".into(), v.to_string().into());
                }
            }
            Action::Cancel(req, batch) => {
                if batch == false {
                    parameters.insert("category".into(), req.category.as_str().into());
                }
                parameters.insert("symbol".into(), req.symbol.into());
                if let Some(v) = req.order_id {
                    parameters.insert("orderId".into(), v.into());
                }
                if let Some(v) = req.order_link_id {
                    parameters.insert("orderLinkId".into(), v.into());
                }
                if let Some(v) = req.order_filter {
                    parameters.insert("orderFilter".into(), v.into());
                }
            }
        }
        parameters
    }
}

pub fn build_ws_orders<'a>(orders: RequestType) -> Value {
    let mut order_array = Vec::new();
    match orders {
        RequestType::Create(req) => {
            for v in req.requests {
                let action = Action::Order(v, false);
                let order_object = Trader::build_orders(action); // Assuming this returns the correct object structure
                let built_order = json!(order_object);
                order_array.push(built_order);
            }
            Value::Array(order_array)
        }
        RequestType::Amend(req) => {
            for v in req.requests {
                let action = Action::Amend(v, false);
                let order_object = Trader::build_orders(action); // Assuming this returns the correct object structure
                let built_order = json!(order_object);
                order_array.push(built_order);
            }
            Value::Array(order_array)
        }
        RequestType::Cancel(req) => {
            for v in req.requests {
                let action = Action::Cancel(v, false);
                let order_object = Trader::build_orders(action); // Assuming this returns the correct object structure
                let built_order = json!(order_object);
                order_array.push(built_order);
            }
            Value::Array(order_array)
        }
    }
}
