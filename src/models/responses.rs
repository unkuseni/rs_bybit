use crate::prelude::*;

/// Response structure for collateral information requests.
///
/// Returned by the `/v5/account/collateral-info` endpoint, this struct provides details about coins available as collateral for margin trading. Bots use this to assess collateral eligibility, borrowing limits, and margin requirements.
pub type CollateralInfoResponse = BybitApiResponse<CollateralInfoList>;

/// An opaque response where the payload is `serde_json::Value`. Used internally by decoding
/// to check ret_code and then conditionally try to deserialize the `result` field.
pub type BybitApiResponseOpaquePayload = BybitApiResponse<Value, Value, u64>;

/// Represents the response from Bybit's server time endpoint.
///
/// This struct is returned when querying the server time via the Bybit API (`/v5/market/time`). It provides the current server time, which is critical for synchronizing trading bot operations with Bybit's server to avoid timestamp-related errors in API requests.
pub type ServerTimeResponse = BybitApiResponse<ServerTime>;

/// Response structure for Kline (candlestick) data requests.
///
/// Returned by the `/v5/market/kline` endpoint, this struct contains Kline data for a specified trading pair and interval. For perpetual futures, Kline data is essential for technical indicators (e.g., moving averages, RSI) used in trading bots.
pub type KlineResponse = BybitApiResponse<KlineSummary>;

/// Response structure for Mark Price Kline data requests.
///
/// Returned by the `/v5/market/mark-price-kline` endpoint, this struct provides Kline data based on the mark price, which is a reference price used in perpetual futures to avoid manipulation and ensure fair liquidation. Mark price Klines are critical for bots to assess funding rates and liquidation risks.
pub type MarkPriceKlineResponse = BybitApiResponse<MarkPriceKlineSummary>;

/// Response structure for Index Price Kline data requests.
///
/// Returned by the `/v5/market/index-price-kline` endpoint, this struct provides Kline data based on the index price, which tracks the underlying asset’s spot price across multiple exchanges. For perpetual futures, the index price is used to anchor the mark price, ensuring it reflects market conditions.
pub type IndexPriceKlineResponse = BybitApiResponse<IndexPriceKlineSummary>;

/// Response structure for Premium Index Price Kline data requests.
///
/// Returned by the `/v5/market/premium-index-price-kline` endpoint, this struct provides Kline data based on the premium index price, which reflects the premium or discount of the perpetual futures price relative to the spot index price. This is key for understanding funding rate dynamics.
pub type PremiumIndexPriceKlineResponse = BybitApiResponse<PremiumIndexPriceKlineSummary>;

/// Response structure for instrument information requests.
///
/// Returned by the `/v5/market/instruments-info` endpoint, this struct provides details about trading instruments, including perpetual futures contracts. Bots use this to configure trading parameters like leverage and order sizes.
pub type InstrumentInfoResponse = BybitApiResponse<InstrumentInfo>;

/// Response structure for order book data requests.
///
/// Returned by the `/v5/market/orderbook` endpoint, this struct provides the current order book for a trading pair, including bid and ask levels. Bots use this for liquidity analysis and to optimize order placement in perpetual futures.
pub type OrderBookResponse = BybitApiResponse<OrderBook>;

/// Response structure for ticker data requests.
///
/// Returned by the `/v5/market/tickers` endpoint, this struct provides real-time market data for a trading pair, such as current price, volume, and funding rates. For perpetual futures, this is critical for monitoring market conditions and funding costs.
pub type TickerResponse = BybitApiResponse<TickersInfo>;

/// Response structure for funding rate history requests.
///
/// Returned by the `/v5/market/funding/history` endpoint, this struct provides historical funding rate data for a perpetual futures contract. Bots use this to analyze funding costs and optimize position timing.
pub type FundingRateResponse = BybitApiResponse<FundingRateSummary>;

/// Response structure for recent trading records requests.
///
/// Returned by the `/v5/market/recent-trade` endpoint, this struct provides a list of recently executed trades for a specified trading pair. Bots use this data to analyze trade volume, price trends, and market sentiment in perpetual futures, which can inform short-term trading decisions.
pub type RecentTradesResponse = BybitApiResponse<RecentTrades>;

/// Response structure for open interest data requests.
///
/// Returned by the `/v5/market/open-interest` endpoint, this struct provides historical or real-time open interest data for a perpetual futures contract. Bots use this to gauge market sentiment, as high open interest often indicates strong participation and potential for larger price moves.
pub type OpenInterestResponse = BybitApiResponse<OpenInterestSummary>;

/// Represents the response from Bybit’s historical volatility API endpoint.
/// Contains metadata and a list of volatility data points for the requested cryptocurrency.
/// This is critical for bots analyzing price stability in perpetual futures markets.
pub type HistoricalVolatilityResponse = BybitApiResponse<Vec<HistoricalVolatility>>;

/// Represents the response from Bybit’s insurance fund API endpoint.
/// The insurance fund covers losses from bankrupt positions in perpetual futures, ensuring
/// market stability. Bots use this to assess exchange risk and counterparty exposure.
pub type InsuranceResponse = BybitApiResponse<InsuranceSummary>;

/// Represents the response from Bybit’s risk limit API endpoint.
/// Provides details on position size and leverage constraints for perpetual futures trading.
pub type RiskLimitResponse = BybitApiResponse<RiskLimitSummary>;

/// Represents the response from Bybit’s delivery price API endpoint.
/// Delivery prices apply to futures with settlement dates, but for perpetual futures, this may
/// be used for reference or historical analysis by bots.
pub type DeliveryPriceResponse = BybitApiResponse<DeliveryPriceSummary>;

/// Represents the response from Bybit’s long/short ratio API endpoint.
/// The long/short ratio shows the balance of bullish vs. bearish positions in the market,
/// critical for sentiment analysis in perpetual futures trading.
pub type LongShortRatioResponse = BybitApiResponse<LongShortRatioSummary>;

/// Represents the response from Bybit's system status API endpoint.
/// This endpoint returns information about platform maintenance or service incidents.
pub type SystemStatusResponse = BybitApiResponse<SystemStatusResult>;

/// Represents the response from Bybit when amending an order.
///
/// This struct captures the result of an order amendment request, including success
/// status and updated order details. In perpetual futures, amending orders (e.g.,
/// changing price, quantity, or TP/SL) is common to adapt to market conditions. Bots
/// should check `ret_code` and `ret_msg` to confirm success and handle errors (e.g.,
/// invalid price, insufficient margin). The `result` field provides the updated order
/// status, which bots can use to track changes.
pub type AmendOrderResponse = BybitApiResponse<OrderStatus>;

/// Represents the response from Bybit when canceling an order.
///
/// This struct captures the result of an order cancellation request. Bots should check
/// `ret_code` and `ret_msg` to confirm success and handle errors (e.g., order already
/// filled). The `result` field provides the canceled order’s status, aiding in state
/// management for perpetual futures trading.
pub type CancelOrderResponse = BybitApiResponse<OrderStatus>;

/// Represents the response from Bybit when querying open orders.
///
/// This struct captures the list of open orders and metadata. Bots should parse the
/// `result` field to track active orders and use `next_page_cursor` for pagination
/// in perpetual futures trading. Check `ret_code` and `ret_msg` to handle errors.
pub type OpenOrdersResponse = BybitApiResponse<OrderHistory>;

/// Represents the response from Bybit when placing an order.
///
/// This struct captures the result of an order placement request. Bots should check
/// `ret_code` and `ret_msg` to confirm success and handle errors (e.g., insufficient
/// margin). The `result` field provides the new order’s status for tracking in
/// perpetual futures trading.
pub type OrderResponse = BybitApiResponse<OrderStatus>;

/// Represents the response from Bybit when querying order history.
///
/// This struct captures the list of past orders and metadata. Bots should parse the
/// `result` field to analyze executed orders and use `next_page_cursor` for pagination
/// in perpetual futures trading. Check `ret_code` and `ret_msg` to handle errors.
pub type OrderHistoryResponse = BybitApiResponse<OrderHistory>;

/// Response structure for the cancel-all orders request.
///
/// Returned by the `/v5/order/cancel-all` endpoint, this struct provides the result of canceling all open orders. Bots use this to confirm successful cancellations and handle errors, ensuring proper risk management in perpetual futures trading.
pub type CancelAllResponse = BybitApiResponse<CancelledList>;

/// Response structure for trade history requests.
///
/// Returned by the `/v5/execution/list` endpoint, this struct provides historical trade execution data for a trading pair. Bots use this to analyze past trades, calculate realized P&L, and refine trading strategies for perpetual futures.
pub type TradeHistoryResponse = BybitApiResponse<TradeHistorySummary>;

/// Response structure for setting margin mode requests.
///
/// Returned by the `/v5/account/set-margin-mode` endpoint, this struct confirms the result of changing the account’s margin mode (e.g., isolated or cross). Bots use this to verify successful configuration and handle errors.
pub type SetMarginModeResponse = BybitApiResponse<MarginModeResult>;

/// Response structure for transaction log requests.
///
/// Returned by the `/v5/account/transaction-log` endpoint, this struct provides historical transaction data for an account. Bots use this to audit trades, calculate costs, and optimize strategies.
pub type TransactionLogResponse = BybitApiResponse<TransactionLogResult>;

/// Response structure for Self-Managed Portfolio (SMP) group requests.
///
/// Returned by the `/v5/account/smp-group` endpoint, this struct confirms the SMP group configuration for the account. Bots use this to verify copy trading or portfolio management settings.
pub type SmpResponse = BybitApiResponse<SmpResult>;

/// Response structure for account information requests.
///
/// Returned by the `/v5/account/info` endpoint, this struct provides details about the account's margin mode, unified margin status, and other configuration settings. Bots use this to verify account settings, manage margin modes, and ensure compliance with trading strategies in perpetual futures trading.
pub type AccountInfoResponse = BybitApiResponse<AccountInfo, Empty, Option<u64>>;

/// Response structure for fee rate requests.
///
/// Returned by the `/v5/account/fee-rate` endpoint, this struct provides the maker and taker fee rates for trading pairs. Bots use this to calculate trading costs and optimize execution strategies in perpetual futures trading.
pub type FeeRateResponse = BybitApiResponse<FeeRateList>;

/// Response structure for wallet balance requests.
///
/// Returned by the `/v5/account/wallet-balance` endpoint, this struct provides the current wallet balance and margin details for an account. Bots use this to monitor account health, calculate available margin, and manage risk in perpetual futures trading.
pub type WalletResponse = BybitApiResponse<WalletList>;

/// Response structure for Unified Trading Account (UTA) status updates.
///
/// Returned by the `/v5/account/set-unified-margin` endpoint, this struct confirms the result of updating the UTA status (e.g., enabling or disabling unified margin). Bots use this to verify successful configuration and handle errors, ensuring proper account settings for perpetual futures trading.
pub type UTAResponse = BybitApiResponse<UTAUpdateStatus>;

/// Response structure for borrow history requests.
///
/// Returned by the `/v5/account/borrow-history` endpoint, this struct provides historical borrowing data for an account. Bots use this to analyze borrowing costs and manage leverage effectively.
pub type BorrowHistoryResponse = BybitApiResponse<BorrowHistory>;

/// Response structure for repaying borrowed liabilities.
///
/// Returned by the `/v5/account/repay-liability` endpoint, this struct confirms the result of repaying borrowed funds for an account. Bots use this to verify successful repayments and update account balance tracking.
pub type RepayLiabilityResponse = BybitApiResponse<LiabilityQty>;

/// Response structure for setting collateral coin status.
///
/// Returned by the `/v5/account/set-collateral-coin` endpoint, this struct confirms the result of enabling or disabling a coin as collateral for margin trading. Bots use this to verify successful configuration and handle errors, ensuring proper collateral management.
pub type SetCollateralCoinResponse = BybitApiResponse<Empty>;

/// Response structure for batch setting collateral coin statuses.
///
/// Returned by the `/v5/account/batch-set-collateral-coin` endpoint, this struct confirms the result of enabling or disabling multiple coins as collateral. Bots use this to verify successful batch configuration and handle errors.
pub type BatchSetCollateralCoinResponse = BybitApiResponse<SwitchList>;

/// Response structure for position information requests.
///
/// Returned by the `/v5/position/list` endpoint, this struct provides details of current open positions. Bots use this to monitor position size, unrealized P&L, and risk metrics in perpetual futures.
pub type InfoResponse = BybitApiResponse<InfoResult>;

/// Response structure for leverage setting requests.
///
/// Returned by the `/v5/position/set-leverage` endpoint, this struct confirms the result of setting leverage for a trading pair. Bots use this to verify successful leverage adjustments and handle errors.
pub type LeverageResponse = BybitApiResponse<Empty>;

/// Response structure for margin change requests.
///
/// Returned by the `/v5/position/switch-margin` endpoint, this struct confirms the result of changing margin settings for a position. Bots use this to verify successful margin adjustments and handle errors.
pub type ChangeMarginResponse = BybitApiResponse<Empty>;

/// Response structure for margin mode setting requests.
///
/// Returned by the `/v5/account/set-margin-mode` endpoint, this struct confirms the result of setting the margin mode. Bots use this to verify successful mode changes and handle errors.
pub type MarginModeResponse = BybitApiResponse<Empty>;

/// Response structure for risk limit setting requests.
///
/// Returned by the `/v5/position/set-risk-limit` endpoint, this struct confirms the result of setting risk limits for a position. Bots use this to verify successful risk limit adjustments and handle errors.
pub type SetRiskLimitResponse = BybitApiResponse<SetRiskLimitResult>;

/// Response structure for trading stop setting requests.
///
/// Returned by the `/v5/position/trading-stop` endpoint, this struct confirms the result of setting take-profit and stop-loss
pub type TradingStopResponse = BybitApiResponse<Empty>;

/// Response structure for auto-add margin setting requests.
///
/// Returned by the `/v5/position/set-auto-add-margin` endpoint, this struct confirms the result of enabling or disabling auto-add margin. Bots use this to verify successful configuration and handle errors, ensuring proper margin management.
pub type AddMarginResponse = BybitApiResponse<Empty>;

/// Response structure for margin adjustment requests.
///
/// Returned by the `/v5/position/add-margin` endpoint, this struct provides the result of adding or reducing margin for a position. Bots use this to confirm the new position state, including updated margin and risk metrics, and handle errors.
pub type AddReduceMarginResponse = BybitApiResponse<AddReduceMarginResult>;

/// Response structure for closed P&L data requests.
///
/// Returned by the `/v5/position/closed-pnl` endpoint, this struct provides historical P&L data for closed positions. Bots use this to analyze realized profits and losses, enabling performance evaluation and strategy optimization.
pub type ClosedPnlResponse = BybitApiResponse<ClosedPnlResult>;

/// Response structure for position move requests.
///
/// Returned by the `/v5/position/move-position` endpoint, this struct confirms the result of moving a position between accounts. Bots use this to verify successful transfers and handle errors, ensuring accurate portfolio management.
pub type MovePositionResponse = BybitApiResponse<MovePositionResult>;

/// Response structure for position move history requests.
///
/// Returned by the `/v5/position/move-history` endpoint, this struct provides historical data on position transfers between accounts. Bots use this to audit transfers and verify portfolio changes.
pub type MoveHistoryResponse = BybitApiResponse<MoveHistoryResult>;

/// Response structure for batch order placement.
///
/// Returned by the `/v5/order/create-batch` endpoint, this struct provides the result of placing multiple orders. Bots use this to confirm successful order placements and handle any errors for individual orders.
pub type BatchPlaceResponse = BybitApiResponse<BatchedOrderList, OrderConfirmationList>;

/// Response structure for batch order amendment.
///
/// Returned by the `/v5/order/amend-batch` endpoint, this struct provides the result of amending multiple orders. Bots use this to confirm successful amendments and handle any errors for individual orders.
pub type BatchAmendResponse = BybitApiResponse<AmendedOrderList, OrderConfirmationList>;
