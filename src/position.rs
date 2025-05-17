use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::api::{Position, API};
use crate::client::Client;
use crate::errors::BybitError;
use crate::model::{
    AddMarginRequest, AddMarginResponse, AddReduceMarginRequest, AddReduceMarginResponse,
    ChangeMarginRequest, ChangeMarginResponse, ClosedPnlRequest, ClosedPnlResponse, InfoResponse,
    LeverageRequest, LeverageResponse, MarginModeRequest, MarginModeResponse, MoveHistoryRequest,
    MoveHistoryResponse, MovePositionRequest, MovePositionResponse, PositionRequest, SetRiskLimit,
    SetRiskLimitResponse, TradingStopRequest, TradingStopResponse,
};
use crate::util::{build_json_request, build_request, date_to_milliseconds};

#[derive(Clone)]
pub struct PositionManager {
    pub client: Client,
    pub recv_window: u16,
}

impl PositionManager {
    /// Asynchronously retrieves information about a position based on the provided request.
    ///
    /// # Arguments
    ///
    /// * `req` - The position request containing the category, symbol, base coin, settle coin, and limit.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `PositionInfo` if the operation is successful, or an error if it fails.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::model::{PositionRequest, Category};
    /// use crate::errors::Result;
    /// use crate::api::PositionInfo;
    /// use my_module::PositionManager;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let position_manager = PositionManager::new();
    ///     let request = PositionRequest::new(Category::Linear, Some("symbol"), Some("base_coin"), Some("settle_coin"), Some(10));
    ///     let position_info = position_manager.get_info(request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_info<'b>(&self, req: PositionRequest<'_>) -> Result<InfoResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }
        if let Some(v) = req.base_coin {
            parameters.insert("baseCoin".into(), v.into());
        }
        if let Some(v) = req.settle_coin {
            parameters.insert("settleCoin".into(), v.into());
        }
        if let Some(v) = req.limit {
            parameters.insert("limit".into(), v.to_string());
        }
        let request = build_request(&parameters);
        let response: InfoResponse = self
            .client
            .get_signed(
                API::Position(Position::Information),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    // Sets the leverage for a given symbol.
    ///
    /// # Arguments
    ///
    /// * `req` - The leverage request containing category, symbol, and leverage.
    ///
    /// # Returns
    ///
    /// A result containing the leverage response.
    pub async fn set_leverage<'b>(
        &self,
        req: LeverageRequest<'_>,
    ) -> Result<LeverageResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("buyLeverage".into(), req.leverage.to_string());
        parameters.insert("sellLeverage".into(), req.leverage.to_string());
        let request = build_json_request(&parameters);
        let response: LeverageResponse = self
            .client
            .post_signed(
                API::Position(Position::SetLeverage),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the margin mode.
    ///
    /// # Arguments
    ///
    /// * `req` - The ChangeMarginRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<ChangeMarginResponse> - The result of setting the margin mode.
    pub async fn set_margin_mode<'b>(
        &self,
        req: ChangeMarginRequest<'_>,
    ) -> Result<ChangeMarginResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("tradeMode".into(), req.trade_mode.into());
        let request = build_json_request(&parameters);
        let response: ChangeMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::SwitchIsolated),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the position mode.
    ///
    /// # Arguments
    ///
    /// * `req` - The MarginModeRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<MarginModeResponse> - The result of setting the position mode.
    pub async fn set_position_mode<'b>(
        &self,
        req: MarginModeRequest<'_>,
    ) -> Result<MarginModeResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }
        if let Some(v) = req.coin {
            parameters.insert("coin".into(), v.into());
        }
        parameters.insert("mode".into(), req.mode.into());
        let request = build_json_request(&parameters);
        let response: MarginModeResponse = self
            .client
            .post_signed(
                API::Position(Position::SwitchMode),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the risk limit.
    ///
    /// # Arguments
    ///
    /// * `req` - The SetRiskLimitRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<SetRiskLimitResult> - The result of setting the risk limit.
    pub async fn set_risk_limit<'b>(
        &self,
        req: SetRiskLimit<'_>,
    ) -> Result<SetRiskLimitResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("riskId".into(), req.risk_id.into());
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }
        let request = build_json_request(&parameters);
        let response: SetRiskLimitResponse = self
            .client
            .post_signed(
                API::Position(Position::SetRiskLimit),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the trading stop.
    ///
    /// # Arguments
    ///
    /// * `req` - The TradingStopRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<TradingStopResponse> - The result of setting the trading stop.
    pub async fn set_trading_stop<'b>(
        &self,
        req: TradingStopRequest<'_>,
    ) -> Result<TradingStopResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        if let Some(v) = req.take_profit {
            parameters.insert("takeProfit".into(), v.into());
        }
        if let Some(v) = req.stop_loss {
            parameters.insert("stopLoss".into(), v.into());
        }
        if let Some(v) = req.tp_trigger_by {
            parameters.insert("tpTriggerBy".into(), v.into());
        }
        if let Some(v) = req.sl_trigger_by {
            parameters.insert("slTriggerBy".into(), v.into());
        }
        if let Some(v) = req.tpsl_mode {
            parameters.insert("tpslMode".into(), v.into());
        }
        if let Some(v) = req.tp_order_type {
            parameters.insert("tpOrderType".into(), v.as_str().into());
        }
        if let Some(v) = req.sl_order_type {
            parameters.insert("slOrderType".into(), v.as_str().into());
        }
        if let Some(v) = req.tp_size {
            parameters.insert("tpSize".into(), v.into());
        }
        if let Some(v) = req.sl_size {
            parameters.insert("slSize".into(), v.into());
        }
        if let Some(v) = req.tp_limit_price {
            parameters.insert("tpLimitPrice".into(), v.into());
        }
        if let Some(v) = req.sl_limit_price {
            parameters.insert("slLimitPrice".into(), v.into());
        }
        parameters.insert("positionIdx".into(), req.position_idx.into());
        let request = build_json_request(&parameters);
        let response: TradingStopResponse = self
            .client
            .post_signed(
                API::Position(Position::SetTradingStop),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the auto-add margin mode for a given position.
    ///
    /// # Arguments
    ///
    /// * `req` - The AddMarginRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// A result containing the AddMarginResponse.
    pub async fn set_add_margin<'b>(
        &self,
        req: AddMarginRequest<'_>,
    ) -> Result<AddMarginResponse, BybitError> {
        // Create a new BTreeMap to store the parameters
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // Add the category and symbol parameters
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());

        // Add the autoAddMargin parameter based on the value of `auto_add`
        if req.auto_add {
            parameters.insert("autoAddMargin".into(), 1.into());
        } else {
            parameters.insert("autoAddMargin".into(), 0.into());
        }

        // Add the positionIdx parameter if it is not None
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }

        // Build the JSON request
        let request = build_json_request(&parameters);

        // Send the POST request to the server
        let response: AddMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::SetAutoaddMargin),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        // Return the response
        Ok(response)
    }

    /// Set the auto add margin.
    ///
    /// # Arguments
    ///
    /// * `req` - The AddReduceMarginRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<AddReduceMarginResponse> - The result of setting the auto add margin.
    pub async fn add_or_reduce_margin<'b>(
        &self,
        req: AddReduceMarginRequest<'_>,
    ) -> Result<AddReduceMarginResponse, BybitError> {
        // Create a new BTreeMap to store the parameters
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // Add the category and symbol parameters
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());

        // Add the margin parameter
        parameters.insert("margin".into(), req.margin.into());

        // Add the positionIdx parameter if it is not None
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }

        // Build the JSON request
        let request = build_json_request(&parameters);

        // Send the POST request to the server
        let response: AddReduceMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::AddorReduceMargin),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        // Return the response
        Ok(response)
    }

    pub async fn get_closed_pnl<'b>(
        &self,
        req: ClosedPnlRequest<'_>,
    ) -> Result<ClosedPnlResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }

        if let Some(start_time) = req.start_time {
            parameters
                .entry("startTime".to_owned())
                .or_insert_with(|| start_time.to_string().into());
        }
        if let Some(end_time) = req.end_time {
            parameters
                .entry("endTime".to_owned())
                .or_insert_with(|| end_time.to_string().into());
        }
        if let Some(v) = req.limit {
            parameters.insert("limit".into(), v.into());
        }
        let request = build_request(&parameters);
        let response: ClosedPnlResponse = self
            .client
            .get_signed(
                API::Position(Position::ClosedPnl),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Moves positions from one user to another.
    ///
    /// # Arguments
    ///
    /// * `req` - The request containing the fromUid, toUid, and list of positions to move.
    ///
    /// # Returns
    ///
    /// A result containing the response.
    pub async fn move_position<'b>(
        &self,
        req: MovePositionRequest<'_>,
    ) -> Result<MovePositionResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        // The user id of the account to move the position from.
        parameters.insert("fromUid".into(), req.from_uid.into());
        // The user id of the account to move the position to.
        parameters.insert("toUid".into(), req.to_uid.into());
        // The list of positions to move.
        parameters.insert("list".into(), json!(req.list));
        let request = build_json_request(&parameters);
        let response: MovePositionResponse = self
            .client
            .post_signed(
                API::Position(Position::MovePosition),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Retrieves the history of position movements.
    ///
    /// # Arguments
    ///
    /// * `req` - The request containing the parameters for the history of position movements.
    ///
    /// # Returns
    ///
    /// A result containing the response.
    pub async fn move_position_history<'b>(
        &self,
        req: MoveHistoryRequest<'_>,
    ) -> Result<MoveHistoryResponse, BybitError> {
        // Create a new BTreeMap to hold the parameters.
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // If the category is specified, add it to the parameters.
        if let Some(category) = req.category {
            parameters.insert("category".into(), category.as_str().into());
        }

        // If the symbol is specified, add it to the parameters.
        if let Some(symbol) = req.symbol {
            parameters.insert("symbol".into(), symbol.into());
        }

        // If the start time is specified, convert it to milliseconds and insert it into the parameters.
        if let Some(start_time) = req.start_time {
            parameters
                .entry("startTime".to_owned())
                .or_insert_with(|| start_time.to_string().into());
        }

        // If the end time is specified, convert it to milliseconds and insert it into the parameters.
        if let Some(end_time) = req.end_time {
            parameters
                .entry("endTime".to_owned())
                .or_insert_with(|| end_time.to_string().into());
        }

        // If the status is specified, add it to the parameters.
        if let Some(status) = req.status {
            parameters.insert("status".into(), status.into());
        }

        // If the block trade id is specified, add it to the parameters.
        if let Some(block_trade_id) = req.block_trade_id {
            parameters.insert("blockTradeId".into(), block_trade_id.into());
        }

        // If the limit is specified, add it to the parameters.
        if let Some(limit) = req.limit {
            parameters.insert("limit".into(), limit.into());
        }

        // Build the request using the parameters.
        let request = build_request(&parameters);

        // Send a GET request to the Bybit API to retrieve the history of position movements.
        let response: MoveHistoryResponse = self
            .client
            .get_signed(
                API::Position(Position::MovePositionHistory),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        // Return the response.
        Ok(response)
    }
}
