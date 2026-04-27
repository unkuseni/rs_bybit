pub mod client;
pub mod stream;
pub use stream::*;

use tokio::time::Duration;

use crate::prelude::*;
use tokio::sync::mpsc;

/// Interval at which the WebSocket event loop sends a ping to keep the connection alive.
pub(crate) const PING_INTERVAL: Duration = Duration::from_secs(30);

/// Helper to send an item through an unbounded channel, mapping the error to `BybitError`.
pub(crate) fn send_or_err<T>(sender: &mpsc::UnboundedSender<T>, item: T) -> Result<(), BybitError> {
    sender.send(item).map_err(|e| BybitError::ChannelSendError {
        underlying: e.to_string(),
    })
}
