use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Config {
    pub rest_api_endpoint: Cow<'static, str>,
    pub ws_endpoint: Cow<'static, str>,
    pub recv_window: u64,
}

impl Config {
    pub const DEFAULT_REST_API_ENDPOINT: &'static str = "https://api.bybit.com";
    pub const DEFAULT_WS_ENDPOINT: &'static str = "wss://stream.bybit.com/v5";

    pub fn new(
        rest_api_endpoint: impl AsRef<str>,
        ws_endpoint: impl AsRef<str>,
        recv_window: impl Into<u64>,
    ) -> Self {
        Self {
            rest_api_endpoint: Cow::Owned(rest_api_endpoint.as_ref().to_string()),
            ws_endpoint: Cow::Owned(ws_endpoint.as_ref().to_string()),
            recv_window: recv_window.into(),
        }
    }

    pub const fn default() -> Self {
        Self {
            rest_api_endpoint: Cow::Borrowed(Self::DEFAULT_REST_API_ENDPOINT),
            ws_endpoint: Cow::Borrowed(Self::DEFAULT_WS_ENDPOINT),
            recv_window: 5000,
        }
    }

    pub const fn testnet() -> Self {
        Self {
            rest_api_endpoint: Cow::Borrowed("https://api-testnet.bybit.com"),
            ws_endpoint: Cow::Borrowed("wss://stream-testnet.bybit.com/v5"),
            recv_window: 5000,
        }
    }

    pub fn set_recv_window(self, recv_window: u64) -> Self {
        Self {
            recv_window,
            ..self
        }
    }
}
