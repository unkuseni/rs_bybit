use crate::prelude::*;

/// Manages a WebSocket connection lifecycle.
///
/// Wraps a `WebSocketStream` and provides connect/disconnect with
/// proper WebSocket Close frame semantics.
pub struct WsClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WsClient {
    /// Connect via an existing stream, typically from `Client::wss_connect`.
    pub fn new(stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { stream }
    }

    /// Get a mutable reference to the underlying stream.
    pub fn stream(&mut self) -> &mut WebSocketStream<MaybeTlsStream<TcpStream>> {
        &mut self.stream
    }

    /// Sends a WebSocket Close frame and consumes the connection.
    pub async fn disconnect(&mut self) -> Result<(), BybitError> {
        self.stream
            .close(None)
            .await
            .map_err(|e| BybitError::Base(format!("Error closing WebSocket: {}", e)))?;
        Ok(())
    }
}
