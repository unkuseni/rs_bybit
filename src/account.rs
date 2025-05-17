use std::collections::BTreeMap;

use crate::api::{Account, API};
use crate::client::Client;
use crate::errors::BybitError;
use crate::model::{
    AccountInfoResponse, BatchSetCollateralCoinResponse, BorrowHistoryRequest,
    BorrowHistoryResponse, Category, CollateralInfoResponse, FeeRateResponse,
    RepayLiabilityResponse, SetCollateralCoinResponse, SetMarginModeResponse, SmpResponse,
    SpotHedgingResponse, TransactionLogRequest, TransactionLogResponse, UTAResponse,
    WalletResponse,
};

use serde_json::{json, Value};

use crate::util::{build_json_request, build_request, date_to_milliseconds};

#[derive(Clone)]
pub struct AccountManager {
    pub client: Client,
    pub recv_window: u16,
}

impl AccountManager {
    /// Fetches the wallet balance for a specific account and optional coin.
    ///
    /// # Arguments
    ///
    /// * `account` - The account type.
    /// * `coin` - The optional coin.
    ///
    /// # Returns
    ///
    /// A result containing the wallet balance response or an error.
    pub async fn get_wallet_balance(
        &self,
        account: &str,
        coin: Option<&str>,
    ) -> Result<WalletResponse, BybitError> {
        // Create a new BTreeMap to hold the request parameters.
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // Add the account type parameter.
        parameters.insert("accountType".into(), account.into());

        // Add the coin parameter if it is provided.
        if let Some(c) = coin {
            parameters.insert("coin".into(), c.into());
        }

        // Build the request using the parameters.
        let request = build_request(&parameters);

        // Send the signed request to the Bybit API and await the response.
        let response: WalletResponse = self
            .client
            .get_signed(
                API::Account(Account::Balance),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        // Return the response.
        Ok(response)
    }

    /// Upgrades the current account to UTA.
    ///
    /// This function sends a POST request to the Bybit API to upgrade the current account to UTA
    /// (Unified Trading Account). It awaits the response and returns the result.
    ///
    /// # Returns
    ///
    /// A result containing the UTA response or an error.
    pub async fn upgrade_to_uta(&self) -> Result<UTAResponse, BybitError> {
        // Send a signed POST request to the Bybit API to upgrade the account to UTA.
        // The request does not require any additional parameters.
        // The response is deserialized into the `UTAResponse` struct.
        // The `await?` operator awaits the response and returns an error if the request fails.

        // Send the request and await the response.
        let response: UTAResponse = self
            .client
            .post_signed(
                API::Account(Account::UpgradetoUTA),
                self.recv_window.into(),
                None,
            )
            .await?;

        // Return the response.
        Ok(response)
    }

    /// Retrieves the borrow history for the current account.
    ///
    /// This function sends a signed GET request to the Bybit API to retrieve the borrow history for
    /// the current account. The request can be filtered by the `coin` and `start_time` parameters.
    /// The response is deserialized into the `BorrowHistoryResponse` struct.
    /// The `await?` operator awaits the response and returns an error if the request fails.
    ///
    /// # Arguments
    ///
    /// * `req` - A `BorrowHistoryRequest` struct containing the parameters for the request.
    ///
    /// # Returns
    ///
    /// A result containing the borrow history response or an error.
    pub async fn get_borrow_history<'b>(
        &self,
        req: BorrowHistoryRequest<'_>,
    ) -> Result<BorrowHistoryResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(c) = req.coin {
            parameters.insert("coin".into(), c.into());
        }

        // If the start time is specified, convert it to milliseconds and insert it into the parameters.
        if let Some(start_time) = req.start_time {
            parameters
                .entry("startTime".to_owned())
                .or_insert_with(|| start_time.into());
        }

        // If the end time is specified, convert it to milliseconds and insert it into the parameters.
        if let Some(end_time) = req.end_time {
            parameters
                .entry("endTime".to_owned())
                .or_insert_with(|| end_time.into());
        }

        // If the limit is specified, insert it into the parameters.
        if let Some(s) = req.limit {
            parameters.insert("limit".into(), s.into());
        }

        let request = build_request(&parameters);
        let response: BorrowHistoryResponse = self
            .client
            .get_signed(
                API::Account(Account::BorrowHistory),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        Ok(response)
    }

    /// Repays liability for a specific coin.
    ///
    /// # Arguments
    ///
    /// * `coin` - The coin for which to repay liability. If not specified, all coins are repaid.
    ///
    /// # Returns
    ///
    /// A result containing a response object or an error.
    pub async fn repay_liability(
        &self,
        coin: Option<&str>,
    ) -> Result<RepayLiabilityResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(c) = coin {
            parameters.insert("coin".into(), c.into());
        }
        let request = build_json_request(&parameters);
        let response: RepayLiabilityResponse = self
            .client
            .post_signed(
                API::Account(Account::RepayLiability),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Sets the collateral coin for a specific coin with the given switch value.
    ///
    /// # Arguments
    ///
    /// * `coin` - The coin for which to set the collateral.
    /// * `switch` - The switch value indicating whether to turn collateral on or off.
    ///
    /// # Returns
    ///
    /// A result containing the set collateral response or an error.
    pub async fn set_collateral_coin(
        &self,
        coin: &str,
        switch: bool,
    ) -> Result<SetCollateralCoinResponse, BybitError> {
        // Create a new BTreeMap to hold the request parameters.
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        // Insert the coin parameter.
        parameters.insert("coin".into(), coin.into());
        // Insert the collateral switch parameter based on the switch value.
        parameters.insert(
            "collateralSwitch".into(),
            if switch { "ON".into() } else { "OFF".into() },
        );
        // Build the request using the parameters.
        let request = build_json_request(&parameters);
        // Send the signed request to the Bybit API and await the response.
        let response: SetCollateralCoinResponse = self
            .client
            .post_signed(
                API::Account(Account::SetCollateral),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        // Return the response.
        Ok(response)
    }

    /// Sets the collateral coin for multiple coins in a single request.
    ///
    /// # Arguments
    ///
    /// * `requests` - A vector of tuples, where each tuple contains the coin and the switch value.
    ///
    /// # Returns
    ///
    /// A result containing the batch set collateral response or an error.
    pub async fn batch_set_collateral(
        &self,
        requests: Vec<(&str, bool)>,
    ) -> Result<BatchSetCollateralCoinResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        let mut requests_array: Vec<Value> = Vec::new();
        // Iterate over the requests and build the request array
        for (coin, switch) in requests {
            let mut build_switch: BTreeMap<String, Value> = BTreeMap::new();
            build_switch.insert("coin".into(), coin.into());
            build_switch.insert("collateralSwitch".into(), switch.into());
            let build_switches = json!(&build_switch);
            requests_array.push(build_switches);
        }
        // Add the request array to the parameters
        parameters.insert("request".into(), Value::Array(requests_array));
        // Build the request
        let request = build_json_request(&parameters);
        // Send the signed request to the Bybit API and await the response
        let response: BatchSetCollateralCoinResponse = self
            .client
            .post_signed(
                API::Account(Account::BatchSetCollateral),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        // Return the response
        Ok(response)
    }

    /// Retrieves the collateral information for a specific coin.
    ///
    /// # Arguments
    ///
    /// * `coin` - The optional coin for which to retrieve the collateral information. If not specified,
    /// information for all coins is returned.
    ///
    /// # Returns
    ///
    /// A result containing the collateral information response or an error.
    pub async fn get_collateral_info(
        &self,
        coin: Option<&str>,
    ) -> Result<CollateralInfoResponse, BybitError> {
        // Create a new BTreeMap to hold the request parameters.
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // If a coin is specified, insert it into the parameters.
        if let Some(v) = coin {
            parameters.insert("currency".into(), v.into());
        }

        // Build the request using the parameters.
        let req = build_request(&parameters);

        // Send the signed request to the Bybit API and await the response.
        let response: CollateralInfoResponse = self
            .client
            .get_signed(
                API::Account(Account::CollateralInfo),
                self.recv_window.into(),
                Some(req),
            )
            .await?;

        // Return the response.
        Ok(response)
    }

    /// Retrieves the fee rate for a given market category and symbol.
    ///
    /// # Arguments
    ///
    /// * `category` - The market category to fetch the fee rate from.
    /// * `symbol` - The trading symbol to fetch the fee rate for. If not specified, the fee rate for all symbols in the category is returned.
    ///
    /// # Returns
    ///
    /// A result containing the fee rate response or an error.
    pub async fn get_fee_rate(
        &self,
        category: Category,
        symbol: Option<String>,
    ) -> Result<FeeRateResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // Insert the category parameter.
        parameters.insert("category".into(), category.as_str().into());

        // If a symbol is specified, insert it into the parameters.
        if let Some(s) = symbol {
            parameters.insert("symbol".into(), s.into());
        }

        // Build the request using the parameters.
        let req = build_request(&parameters);

        // Send the signed request to the Bybit API and await the response.
        let response: FeeRateResponse = self
            .client
            .post_signed(
                API::Account(Account::FeeRate),
                self.recv_window.into(),
                Some(req),
            )
            .await?;

        // Return the response.
        Ok(response)
    }

    /// Retrieves the account information for the current account.
    ///
    /// This function sends a signed GET request to the Bybit API to retrieve the account information.
    /// The response is deserialized into the `AccountInfoResponse` struct.
    /// The `await?` operator awaits the response and returns an error if the request fails.
    ///
    /// # Returns
    ///
    /// A result containing the account information response or an error.
    pub async fn get_account_info(&self) -> Result<AccountInfoResponse, BybitError> {
        // Send the signed GET request to the Bybit API to retrieve the account information.
        // The request does not require any additional parameters.
        // The response is deserialized into the `AccountInfoResponse` struct.
        // The `await?` operator awaits the response and returns an error if the request fails.

        // Send the request and await the response.
        let response: AccountInfoResponse = self
            .client
            .get_signed(
                API::Account(Account::Information),
                self.recv_window.into(),
                None,
            )
            .await?;

        // Return the response.
        Ok(response)
    }

    /// Retrieves the transaction log for the current account.
    ///
    /// This function sends a signed GET request to the Bybit API to retrieve the transaction log.
    /// The request parameters are serialized into a JSON object and sent in the request body.
    /// The response is deserialized into the `TransactionLogResponse` struct.
    /// The `await?` operator awaits the response and returns an error if the request fails.
    ///
    /// # Arguments
    ///
    /// * `req` - An instance of `TransactionLogRequest` containing the request parameters.
    ///
    /// # Returns
    ///
    /// A result containing the transaction log response or an error.
    pub async fn get_transaction_log<'b>(
        &self,
        req: TransactionLogRequest<'_>,
    ) -> Result<TransactionLogResponse, BybitError> {
        // Create a mutable map to store the request parameters.
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();

        // Add the account type to the request parameters if it is specified.
        if let Some(v) = req.account_type {
            parameters.insert("accountType".into(), v.into());
        }

        // Add the category to the request parameters if it is specified.
        if let Some(v) = req.category {
            parameters.insert("category".into(), v.as_str().into());
        }

        // Add the currency to the request parameters if it is specified.
        if let Some(s) = req.currency {
            parameters.insert("currency".into(), s.into());
        }

        // Add the base coin to the request parameters if it is specified.
        if let Some(c) = req.base_coin {
            parameters.insert("baseCoin".into(), c.into());
        }

        // Add the log type to the request parameters if it is specified.
        if let Some(t) = req.log_type {
            parameters.insert("type".into(), t.into());
        }

        // Add the start time to the request parameters if it is specified.
        if let Some(start_time) = req.start_time {
            parameters
                .entry("startTime".to_owned())
                .or_insert_with(|| start_time.into());
        }

        // Add the end time to the request parameters if it is specified.
        if let Some(end_time) = req.end_time {
            parameters
                .entry("endTime".to_owned())
                .or_insert_with(|| end_time.into());
        }

        // Add the limit to the request parameters if it is specified.
        if let Some(s) = req.limit {
            parameters.insert("limit".into(), s.into());
        }

        // Build the request from the parameters.
        let request = build_request(&parameters);

        // Send the signed GET request to the Bybit API to retrieve the transaction log.
        let response: TransactionLogResponse = self
            .client
            .get_signed(
                API::Account(Account::TransactionLog),
                self.recv_window.into(),
                Some(request),
            )
            .await?;

        // Return the response.
        Ok(response)
    }

    /// Retrieves the Server-Market-Portfolio (SMP) group ID for the current user.
    ///
    /// # Returns
    ///
    /// A result containing the SMP response or an error.
    pub async fn get_smp_id(&self) -> Result<SmpResponse, BybitError> {
        // Send a signed GET request to the Bybit API to retrieve the SMP group ID.
        // The request does not require any additional parameters.
        let response: SmpResponse = self
            .client
            .get_signed(
                API::Account(Account::SMPGroupID),
                self.recv_window.into(),
                None,
            )
            .await?;

        // Return the response.
        Ok(response)
    }

    /// Sets the margin mode for the current account.
    ///
    /// # Arguments
    ///
    /// * `margin_mode` - The desired margin mode to set. Can be "CROSS" or "FIXED".
    ///
    /// # Returns
    ///
    /// A result containing the set margin mode response or an error.
    pub async fn set_margin_mode(
        &self,
        margin_mode: &str,
    ) -> Result<SetMarginModeResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("setMarginMode".into(), margin_mode.into());
        let request = build_json_request(&parameters);
        let response: SetMarginModeResponse = self
            .client
            .post_signed(
                API::Account(Account::SetMarginMode),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Sets the spot hedging mode for the current account.
    ///
    /// # Arguments
    ///
    /// * `spot_hedging` - The desired spot hedging mode. `true` sets the mode to "ON",
    ///   `false` sets the mode to "OFF".
    ///
    /// # Returns
    ///
    /// A result containing the set spot hedging mode response or an error.
    pub async fn set_spot_hedging(
        &self,
        spot_hedging: bool,
    ) -> Result<SpotHedgingResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert(
            "setHedgingMode".into(),
            if spot_hedging {
                "ON".into()
            } else {
                "OFF".into()
            },
        );
        let request = build_json_request(&parameters);
        let response: SpotHedgingResponse = self
            .client
            .post_signed(
                API::Account(Account::SetSpotHedging),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }
}
