use crate::account::AccountManager;
use crate::asset::AssetManager;
use crate::client::Client;
use crate::config::Config;
use crate::general::General;
use crate::market::MarketData;
use crate::position::PositionManager;
use crate::trade::Trader;
use crate::ws::Stream;

pub enum API {
    Market(Market),
    Trade(Trade),
    Position(Position),
    Account(Account),
    Asset(Asset),
    SpotLeverage(SpotLeverage),
    SpotMargin(SpotMargin),
}
/// Bybit Endpoints
#[derive(Debug)]
pub enum WebsocketAPI {
    PublicSpot,
    PublicLinear,
    PublicInverse,
    Private,
    TradeStream,
}

#[derive(Clone)]
pub enum Public {
    Spot,
    Linear,
    Inverse,
}

pub enum Market {
    Time,
    Kline,
    MarkPriceKline,
    IndexPriceKline,
    PremiumIndexPriceKline,
    InstrumentsInfo,
    OrderBook,
    Ticker,
    FundingRate,
    RecentTrades,
    OpenInterest,
    HistoricalVolatility,
    Insurance,
    RiskLimit,
    DeliveryPrice,
    LongShortRatio,
    SystemStatus,
}

pub enum Trade {
    Place,
    Amend,
    Cancel,
    OpenOrders,
    CancelAll,
    History,
    TradeHistory,
    BatchPlace,
    BatchAmend,
    BatchCancel,
    SpotBorrowCheck,
    SetDisconnectCancelall,
}

pub enum Position {
    Information,
    SetLeverage,
    SetRiskLimit,
    SetTradingStop,
    SwitchIsolated,
    SwitchMode,
    SetAutoaddMargin,
    AddorReduceMargin,
    ClosedPnl,
    MovePosition,
    MovePositionHistory,
}

pub enum Account {
    Balance,
    UpgradetoUTA,
    BorrowHistory,
    RepayLiability,
    SetCollateral,
    BatchSetCollateral,
    CollateralInfo,
    CoinGreeks,
    FeeRate,
    Information,
    TransactionLog,
    SetMarginMode,
    SMPGroupID,
    SetSpotHedging,
}

pub enum Asset {
    CoinExchangeRecord,
    DeliveryRecord,
    SettlementRecord,
    Intertransfer,
    QueryTransferList,
    SaveTransferSubmember,
    UniversalTransfer,
    QueryUniversalTransferList,
    QueryTransferCoinList,
    QueryTransferSubmemberList,
    QueryAccountCoinBalance,
    QueryAssetInfo,
    QueryAllowedList,
    QueryRecord,
    QueryInfo,
    QueryAsset,
    Withdraw,
    CancelWithdraw,
    Deposit,
    QuerySubmemberAddress,
    OrderRecord,
}

pub enum SpotLeverage {
    Info,
    Marketinfo,
    Purchase,
    Redeem,
    OrderRecord,
}

pub enum SpotMargin {
    SwitchMode,
    SetLeverage,
    MarginCoinInfo,
    State,
    BorrowableCoin,
    LoanInfo,
    LoanAccountInfo,
    Borrow,
    Repay,
    BorrowOrderDetail,
    RepayOrderDetail,
    ClassicMarginToggle,
}

impl AsRef<str> for API {
    fn as_ref(&self) -> &str {
        match self {
            API::Market(route) => match route {
                Market::Time => "/v5/market/time",
                Market::Insurance => "/v5/market/insurance",
                Market::Kline => "/v5/market/kline",
                Market::MarkPriceKline => "/v5/market/mark-price-kline",
                Market::IndexPriceKline => "/v5/market/index-price-kline",
                Market::PremiumIndexPriceKline => "/v5/market/premium-index-price-kline",
                Market::InstrumentsInfo => "/v5/market/instruments-info",
                Market::OrderBook => "/v5/market/orderbook",
                Market::Ticker => "/v5/market/tickers",
                Market::RecentTrades => "/v5/market/recent-trade",
                Market::FundingRate => "/v5/market/funding/history",
                Market::OpenInterest => "/v5/market/open-interest",
                Market::HistoricalVolatility => "/v5/market/historical-volatility",
                Market::RiskLimit => "/v5/market/risk-limit",
                Market::DeliveryPrice => "/v5/market/delivery-price",
                Market::LongShortRatio => "/v5/market/account-ratio",
                Market::SystemStatus => "/v5/system/status",
            },
            API::Trade(route) => match route {
                Trade::Place => "/v5/order/create",
                Trade::Amend => "/v5/order/amend",
                Trade::Cancel => "/v5/order/cancel",
                Trade::OpenOrders => "/v5/order/realtime",
                Trade::CancelAll => "/v5/order/cancel-all",
                Trade::History => "/v5/order/history",
                Trade::TradeHistory => "/v5/execution/list",
                Trade::BatchPlace => "/v5/order/create-batch",
                Trade::BatchAmend => "/v5/order/amend-batch",
                Trade::BatchCancel => "/v5/order/cancel-batch",
                Trade::SpotBorrowCheck => "/v5/order/spot-borrow-check",
                Trade::SetDisconnectCancelall => "/v5/order/disconnected-cancel-all",
            },
            API::Position(route) => match route {
                Position::Information => "/v5/position/list",
                Position::SetLeverage => "/v5/position/set-leverage",
                Position::SwitchIsolated => "/v5/position/switch-isolated",
                Position::SwitchMode => "/v5/position/switch-mode",
                Position::SetRiskLimit => "/v5/position/set-risk-limit",
                Position::SetTradingStop => "/v5/position/trading-stop",
                Position::SetAutoaddMargin => "/v5/position/set-auto-add-margin",
                Position::AddorReduceMargin => "/v5/position/add-margin",
                Position::ClosedPnl => "/v5/position/closed-pnl",
                Position::MovePosition => "/v5/position/move-positions",
                Position::MovePositionHistory => "/v5/position/move-history",
            },
            API::Account(route) => match route {
                Account::Balance => "/v5/account/wallet-balance",
                Account::UpgradetoUTA => "/v5/account/upgrade-to-uta",
                Account::BorrowHistory => "/v5/account/borrow-history",
                Account::RepayLiability => "/v5/account/quick-repayment",
                Account::SetCollateral => "/v5/account/set-collateral-switch",
                Account::BatchSetCollateral => "/v5/account/set-collateral-switch-batch",
                Account::CollateralInfo => "/v5/account/collateral-info",
                Account::CoinGreeks => "/v5/asset/coin-greeks",
                Account::FeeRate => "/v5/account/fee-rate",
                Account::Information => "/v5/account/info",
                Account::TransactionLog => "/v5/account/transaction-log",
                Account::SMPGroupID => "/v5/account/smp-group",
                Account::SetMarginMode => "/v5/aaccount/set-margin-mode",
                Account::SetSpotHedging => "/v5/account/set-hedging-mode",
            },
            API::Asset(route) => match route {
                Asset::CoinExchangeRecord => "/v5/asset/exchange/order-record",
                Asset::DeliveryRecord => "/v5/asset/delivery-record",
                Asset::SettlementRecord => "/v5/asset/settlement-record",
                Asset::QueryAssetInfo => "/v5/asset/transfer/query-asset-info",
                Asset::QueryAccountCoinBalance => "/v5/asset/transfer/query-account-coins-balance",
                Asset::QueryTransferCoinList => "/v5/asset/transfer/query-transfer-coin-list",
                Asset::Intertransfer => "/v5/asset/transfer/inter-transfer",
                Asset::QueryTransferList => "/v5/asset/transfer/query-inter-transfer-list",
                Asset::QueryTransferSubmemberList => "/v5/asset/transfer/query-sub-member-list",
                Asset::UniversalTransfer => "/v5/asset/transfer/universal-transfer",
                Asset::QueryUniversalTransferList => {
                    "/v5/asset/transfer/query-universal-transfer-list"
                }
                Asset::QueryAllowedList => "/v5/asset/deposit/query-allowed-list",
                Asset::Withdraw => "/v5/asset/withdraw/create",
                Asset::CancelWithdraw => "/v5/asset/withdraw/cancel",
                Asset::QueryInfo => "/v5/asset/coin/query-info",
                Asset::QueryRecord => "/v5/asset/deposit/query-record",
                Asset::QuerySubmemberAddress => "/v5/asset/deposit/query-sub-member-address",
                _ => {
                    todo!("Asset route not implemented");
                }
            },
            API::SpotLeverage(route) => match route {
                SpotLeverage::Info => "/v5/spot-lever-token/info",
                SpotLeverage::Marketinfo => "v5/spot-lever-token/reference",
                SpotLeverage::Purchase => "/v5/spot-lever-token/purchase",
                SpotLeverage::Redeem => "/v5/spot-lever-token/redeem",
                SpotLeverage::OrderRecord => "/v5/spot-lever-token/order-record",
            },
            API::SpotMargin(route) => match route {
                SpotMargin::SwitchMode => "/v5/spot-margin-trade/switch-mode",
                SpotMargin::SetLeverage => "/v5/spot-margin-trade/set-leverage",
                SpotMargin::State => "/v5/spot-margin-trade/state",
                SpotMargin::MarginCoinInfo => "/v5/spot-cross-margin-trade/pledge-token",
                SpotMargin::BorrowableCoin => "/v5/spot-cross-margin-trade/borrow-token",
                SpotMargin::LoanInfo => "/v5/spot-cross-margin-trade/loan",
                SpotMargin::LoanAccountInfo => "/v5/spot-cross-margin-trade/account",
                SpotMargin::Borrow => "/v5/spot-cross-margin-trade/loan",
                SpotMargin::Repay => "/v5/spot-cross-margin-trade/repay",
                SpotMargin::BorrowOrderDetail => "/v5/spot-cross-margin-trade/orders",
                SpotMargin::RepayOrderDetail => "/v5/spot-cross-margin-trade/repay-history",
                SpotMargin::ClassicMarginToggle => "/v5/spot-cross-margin-trade/switch",
            },
        }
    }
}

impl AsRef<str> for WebsocketAPI {
    fn as_ref(&self) -> &str {
        match self {
            WebsocketAPI::PublicSpot => "/public/spot",
            WebsocketAPI::PublicLinear => "/public/linear",
            WebsocketAPI::PublicInverse => "/public/inverse",
            WebsocketAPI::Private => "/private",
            WebsocketAPI::TradeStream => "/trade",
        }
    }
}

/// A trait that all modules must implement to be a Bybit API client module.
///
/// This trait provides two methods to create a new instance of the module,
/// one with default configuration and one with custom configuration.
pub trait Bybit {
    /// Creates a new instance of the module with default configuration.
    ///
    /// # Parameters
    ///
    /// * `api_key`: The API key to be used for the module.
    /// * `secret_key`: The secret key to be used for the module.
    ///
    /// # Returns
    ///
    /// A new instance of the module.
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;

    /// Creates a new instance of the module with custom configuration.
    ///
    /// # Parameters
    ///
    /// * `config`: The custom configuration to be used for the module.
    /// * `api_key`: The API key to be used for the module.
    /// * `secret_key`: The secret key to be used for the module.
    ///
    /// # Returns
    ///
    /// A new instance of the module.
    fn new_with_config(
        config: &Config,
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> Self;
}

impl Bybit for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        Self::new_with_config(&Config::default(), api_key, secret_key)
    }

    fn new_with_config(
        config: &Config,
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.to_string()),
        }
    }
}

impl Bybit for MarketData {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> MarketData {
        Self::new_with_config(&Config::default(), api_key, secret_key)
    }
    fn new_with_config(
        config: &Config,
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> MarketData {
        MarketData {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.to_string()),
            recv_window: config.recv_window,
        }
    }
}

impl Bybit for Trader {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Trader {
        Self::new_with_config(&Config::default(), api_key, secret_key)
    }
    fn new_with_config(
        config: &Config,
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> Trader {
        Trader {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.to_string()),
            recv_window: config.recv_window,
        }
    }
}
impl Bybit for PositionManager {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> PositionManager {
        Self::new_with_config(&Config::default(), api_key, secret_key)
    }
    fn new_with_config(
        config: &Config,
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> PositionManager {
        PositionManager {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.to_string()),
            recv_window: config.recv_window,
        }
    }
}

impl Bybit for AccountManager {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> AccountManager {
        Self::new_with_config(&Config::default(), api_key, secret_key)
    }
    fn new_with_config(
        config: &Config,
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> AccountManager {
        AccountManager {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.to_string()),
            recv_window: config.recv_window,
        }
    }
}

impl Bybit for AssetManager {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> AssetManager {
        Self::new_with_config(&Config::default(), api_key, secret_key)
    }
    fn new_with_config(
        config: &Config,
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> AssetManager {
        AssetManager {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.to_string()),
            recv_window: config.recv_window,
        }
    }
}

impl Bybit for Stream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Stream {
        Self::new_with_config(&Config::default(), api_key, secret_key)
    }

    fn new_with_config(
        config: &Config,
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> Stream {
        Stream {
            client: Client::new(api_key, secret_key, config.ws_endpoint.to_string()),
        }
    }
}
