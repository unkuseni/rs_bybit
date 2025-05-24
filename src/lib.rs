mod account;
mod api;
mod asset;
mod client;
mod config;
mod errors;
mod general;
mod market;
mod models;
mod position;
mod serde_helpers;
mod timed;
mod trade;
mod util;
mod ws;

#[cfg(test)]
mod open_fixture;

pub mod prelude {

    pub use crate::account::*;
    pub use crate::api::*;
    pub use crate::asset::*;
    pub use crate::client::*;
    pub use crate::config::*;
    pub use crate::errors::*;
    pub use crate::general::*;
    pub use crate::market::*;
    pub use crate::models::*;
    pub use crate::position::*;
    pub use crate::serde_helpers::*;
    pub use crate::timed::*;
    pub use crate::trade::*;
    pub use crate::util::*;
    pub use crate::ws::*;

    pub(crate) use core::f64;
    pub(crate) use derive_more::{Display, TryUnwrap};
    pub(crate) use futures::sink::SinkExt;
    pub(crate) use hex::encode as hex_encode;
    pub(crate) use hmac::{Hmac, Mac};
    pub(crate) use log::error;
    pub(crate) use reqwest::{
        header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
        Client as ReqwestClient, Response as ReqwestResponse, StatusCode,
    };
    pub(crate) use serde::de::DeserializeOwned;
    pub(crate) use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub(crate) use serde_json::{json, Value};
    pub(crate) use sha2::Sha256;
    pub(crate) use std::any::type_name;
    pub(crate) use std::str::FromStr;
    pub(crate) use std::{borrow::Cow, collections::BTreeMap};
    pub(crate) use thiserror::Error;
    pub(crate) use tokio::net::TcpStream;
    pub(crate) use tokio_tungstenite::WebSocketStream;
    pub(crate) use tokio_tungstenite::{
        connect_async, tungstenite::Message as WsMessage, MaybeTlsStream,
    };
    pub(crate) use url::Url as WsUrl;
}

pub use prelude::*;
