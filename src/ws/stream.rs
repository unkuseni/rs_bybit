use crate::prelude::*;
use crate::ws::client::WsClient;
use crate::ws::{send_or_err, PING_INTERVAL};

use futures::{SinkExt, StreamExt};
use log::trace;
use std::collections::HashMap;
use std::time::Instant;

use tokio::sync::mpsc;

use tokio_tungstenite::tungstenite::Message as WsMessage;

#[derive(Clone)]
pub struct Stream {
    pub client: Client,
}

impl Stream {
    /// Tests for connectivity by sending a ping request to the Bybit server.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `String` with the response message if successful,
    ///
    /// * `private` is set to `true` if the request is for a private endpoint
    /// or a `BybitError` if an error occurs.
    pub async fn ws_ping(&self, private: bool) -> Result<(), BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("req_id".into(), generate_random_uid(8).into());
        parameters.insert("op".into(), "ping".into());
        let request = build_json_request(&parameters);
        let endpoint = if private {
            WebsocketAPI::Private
        } else {
            WebsocketAPI::PublicLinear
        };
        let mut response = self
            .client
            .wss_connect(endpoint, Some(request), private, None)
            .await?;
        let Some(data) = response.next().await else {
            return Err(BybitError::Base(
                "Failed to receive ping response".to_string(),
            ));
        };

        let data = data
            .map_err(|e| BybitError::Base(format!("Failed to get ping response, error {}", e)))?;
        if let WsMessage::Text(data) = data {
            let response: PongResponse = serde_json::from_str(&data)?;
            match response {
                PongResponse::PublicPong(pong) => {
                    trace!("Pong received successfully: {:#?}", pong);
                }
                PongResponse::PrivatePong(pong) => {
                    trace!("Pong received successfully: {:#?}", pong);
                }
            }
        }
        Ok(())
    }

    pub async fn ws_priv_subscribe<'b, F>(
        &self,
        req: Subscription<'_>,
        handler: F,
    ) -> Result<(), BybitError>
    where
        F: FnMut(WebsocketEvents) -> Result<(), BybitError> + 'static + Send,
    {
        let request = Self::build_subscription(req);
        let response = self
            .client
            .wss_connect(WebsocketAPI::Private, Some(request), true, Some(10))
            .await?;
        let mut ws_client = WsClient::new(response);
        Self::event_loop(&mut ws_client, handler, None).await?;
        Ok(())
    }

    /// Connects to a private WebSocket endpoint with dynamic subscription control.
    ///
    /// This method allows bots to dynamically subscribe and unsubscribe to private data streams
    /// (orders, positions, executions, wallet) after the connection is established.
    /// Use this when you need to change private subscriptions without reconnecting.
    ///
    /// # Arguments
    ///
    /// * `cmd_receiver` - Channel receiver for subscription and order commands
    /// * `handler` - Callback function to handle incoming WebSocket events
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the connection runs successfully, or a `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    /// let ws: Stream = Bybit::new(Some(api_key), Some(secret));
    ///
    /// // Start connection
    /// tokio::spawn(async move {
    ///     ws.ws_priv_subscribe_with_commands(cmd_rx, |event| {
    ///         // Handle private events (orders, positions, etc.)
    ///         Ok(())
    ///     }).await.unwrap();
    /// });
    ///
    /// // Dynamically subscribe to private topics
    /// cmd_tx.send(RequestType::Subscribe(Subscription::new(
    ///     "subscribe",
    ///     vec!["order"]
    /// )))?;
    ///
    /// // Later, add more subscriptions
    /// cmd_tx.send(RequestType::Subscribe(Subscription::new(
    ///     "subscribe",
    ///     vec!["position", "execution"]
    /// )))?;
    /// ```
    pub async fn ws_priv_subscribe_with_commands<'a, F>(
        &self,
        cmd_receiver: mpsc::UnboundedReceiver<RequestType<'a>>,
        handler: F,
    ) -> Result<(), BybitError>
    where
        F: FnMut(WebsocketEvents) -> Result<(), BybitError> + 'static + Send,
        'a: 'static,
    {
        let response = self
            .client
            .wss_connect(WebsocketAPI::Private, None, true, Some(10))
            .await?;
        let mut ws_client = WsClient::new(response);
        Self::event_loop(&mut ws_client, handler, Some(cmd_receiver)).await?;
        Ok(())
    }

    pub async fn ws_subscribe<'b, F>(
        &self,
        req: Subscription<'_>,
        category: Category,
        handler: F,
    ) -> Result<(), BybitError>
    where
        F: FnMut(WebsocketEvents) -> Result<(), BybitError> + 'static + Send,
    {
        let endpoint = category.public_ws_endpoint();
        let request = Self::build_subscription(req);
        let response = self
            .client
            .wss_connect(endpoint, Some(request), false, None)
            .await?;
        let mut ws_client = WsClient::new(response);
        Self::event_loop(&mut ws_client, handler, None).await?;
        Ok(())
    }

    /// Connects to a public WebSocket endpoint with dynamic subscription control.
    ///
    /// This method allows bots to dynamically subscribe and unsubscribe to market data streams
    /// after the connection is established. Use this when you need to change subscriptions
    /// without reconnecting.
    ///
    /// # Arguments
    ///
    /// * `category` - The market category (Linear, Inverse, Spot, Option)
    /// * `cmd_receiver` - Channel receiver for subscription and order commands
    /// * `handler` - Callback function to handle incoming WebSocket events
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the connection runs successfully, or a `BybitError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    /// let ws: Stream = Bybit::new(None, None);
    ///
    /// // Start connection
    /// tokio::spawn(async move {
    ///     ws.ws_subscribe_with_commands(Category::Linear, cmd_rx, |event| {
    ///         // Handle events
    ///         Ok(())
    ///     }).await.unwrap();
    /// });
    ///
    /// // Dynamically subscribe to topics
    /// cmd_tx.send(RequestType::Subscribe(Subscription::new(
    ///     "subscribe",
    ///     vec!["orderbook.50.BTCUSDT"]
    /// )))?;
    ///
    /// // Later, unsubscribe
    /// cmd_tx.send(RequestType::Unsubscribe(Subscription::new(
    ///     "unsubscribe",
    ///     vec!["orderbook.50.BTCUSDT"]
    /// )))?;
    /// ```
    pub async fn ws_subscribe_with_commands<'a, F>(
        &self,
        category: Category,
        cmd_receiver: mpsc::UnboundedReceiver<RequestType<'a>>,
        handler: F,
    ) -> Result<(), BybitError>
    where
        F: FnMut(WebsocketEvents) -> Result<(), BybitError> + 'static + Send,
        'a: 'static,
    {
        let endpoint = category.public_ws_endpoint();
        let response = self.client.wss_connect(endpoint, None, false, None).await?;
        let mut ws_client = WsClient::new(response);
        Self::event_loop(&mut ws_client, handler, Some(cmd_receiver)).await?;

        Ok(())
    }

    pub fn build_subscription(action: Subscription) -> String {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("req_id".into(), generate_random_uid(8).into());
        parameters.insert("op".into(), action.op.into());
        let args_value: Value = action
            .args
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .into();
        parameters.insert("args".into(), args_value);

        build_json_request(&parameters)
    }

    pub fn build_trade_subscription(
        orders: RequestType,
        recv_window: Option<u64>,
    ) -> Result<String, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("reqId".into(), generate_random_uid(16).into());
        let mut header_map: BTreeMap<String, String> = BTreeMap::new();
        header_map.insert("X-BAPI-TIMESTAMP".into(), get_timestamp().to_string());
        header_map.insert(
            "X-BAPI-RECV-WINDOW".into(),
            recv_window.unwrap_or(5000).to_string(),
        );
        parameters.insert("header".into(), json!(header_map));
        match orders {
            RequestType::Create(order) => {
                parameters.insert("op".into(), "order.create".into());
                parameters.insert("args".into(), build_ws_orders(RequestType::Create(order)));
            }
            RequestType::CreateBatch(order) => {
                parameters.insert("op".into(), "order.create-batch".into());
                parameters.insert(
                    "args".into(),
                    build_ws_orders(RequestType::CreateBatch(order)),
                );
            }
            RequestType::Amend(order) => {
                parameters.insert("op".into(), "order.amend".into());
                parameters.insert("args".into(), build_ws_orders(RequestType::Amend(order)));
            }
            RequestType::AmendBatch(order) => {
                parameters.insert("op".into(), "order.amend-batch".into());
                parameters.insert(
                    "args".into(),
                    build_ws_orders(RequestType::AmendBatch(order)),
                );
            }
            RequestType::Cancel(order) => {
                parameters.insert("op".into(), "order.cancel".into());
                parameters.insert("args".into(), build_ws_orders(RequestType::Cancel(order)));
            }
            RequestType::CancelBatch(order) => {
                parameters.insert("op".into(), "order.cancel-batch".into());
                parameters.insert(
                    "args".into(),
                    build_ws_orders(RequestType::CancelBatch(order)),
                );
            }
            _ => {
                return Err(BybitError::Base(format!(
                    "Unsupported trade request type: {:?}",
                    std::any::type_name::<RequestType>()
                )));
            }
        }
        Ok(build_json_request(&parameters))
    }

    /// Generic helper that eliminates boilerplate for channel-based subscriptions.
    async fn subscribe_channel<T, M>(
        &self,
        request: Subscription<'_>,
        category: Category,
        sender: mpsc::UnboundedSender<T>,
        mapper: M,
    ) -> Result<(), BybitError>
    where
        T: Send + 'static,
        M: Fn(WebsocketEvents) -> Option<T> + Send + 'static,
    {
        self.ws_subscribe(request, category, move |event| {
            if let Some(data) = mapper(event) {
                send_or_err(&sender, data)?;
            }
            Ok(())
        })
        .await
    }

    /// Subscribes to the specified order book updates and handles the order book events
    ///
    /// # Arguments
    ///
    /// * `subs` - A vector of tuples containing the order book ID and symbol
    /// * `category` - The category of the order book
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate_name::Category;
    /// let subs = vec![(1, "BTC"), (2, "ETH")];
    /// ```
    pub async fn ws_orderbook(
        &self,
        subs: Vec<(i32, &str)>,
        category: Category,
        sender: mpsc::UnboundedSender<OrderBookUpdate>,
    ) -> Result<(), BybitError> {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|(num, sym)| format!("orderbook.{}.{}", num, sym.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(AsRef::as_ref).collect());
        self.subscribe_channel(request, category, sender, |event| {
            if let WebsocketEvents::OrderBookEvent(order_book) = event {
                Some(order_book)
            } else {
                None
            }
        })
        .await
    }

    /// Subscribes to RPI (Real-time Price Improvement) orderbook stream.
    ///
    /// RPI orderbooks show both regular orders and RPI orders, which can provide price improvement for takers.
    /// Push frequency: 100ms for Spot, Perpetual & Futures (level 50 data).
    /// Topic format: `orderbook.rpi.{symbol}`
    ///
    /// # Arguments
    ///
    /// * `subs` - Vector of symbol strings to subscribe to (e.g., `vec!["BTCUSDT", "ETHUSDT"]`)
    /// * `category` - Product category (Linear, Inverse, or Spot)
    /// * `sender` - Channel sender for RPI orderbook updates
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if subscription succeeds, otherwise returns a `BybitError`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rs_bybit::prelude::*;
    /// use tokio::sync::mpsc;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), BybitError> {
    /// let client = Client::new("api_key", "api_secret", None, None)?;
    /// let stream = Stream { client };
    /// let (tx, mut rx) = mpsc::unbounded_channel();
    ///
    /// stream.ws_rpi_orderbook(vec!["BTCUSDT"], Category::Linear, tx).await?;
    ///
    /// while let Some(update) = rx.recv().await {
    ///     println!("RPI Orderbook update: {:?}", update);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ws_rpi_orderbook(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<RPIOrderbookUpdate>,
    ) -> Result<(), BybitError> {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|sym| format!("orderbook.rpi.{}", sym.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(AsRef::as_ref).collect());
        self.subscribe_channel(request, category, sender, |event| {
            if let WebsocketEvents::RPIOrderBookEvent(rpi_order_book) = event {
                Some(rpi_order_book)
            } else {
                None
            }
        })
        .await
    }

    /// This function subscribes to the specified trades and handles the trade events.
    /// # Arguments
    ///
    /// * `subs` - A vector of trade subscriptions
    /// * `category` - The category of the trades
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate_name::Category;
    /// let subs = vec!["BTCUSD", "ETHUSD"];
    /// let category = Category::Linear;
    /// ws_trades(subs, category);
    /// ```
    pub async fn ws_trades(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<WsTrade>,
    ) -> Result<(), BybitError> {
        let arr: Vec<String> = subs
            .iter()
            .map(|&sub| format!("publicTrade.{}", sub.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(AsRef::as_ref).collect());
        let handler = move |event| {
            if let WebsocketEvents::TradeEvent(trades) = event {
                for trade in trades.data {
                    send_or_err(&sender, trade)?;
                }
            }
            Ok(())
        };

        self.ws_subscribe(request, category, handler).await
    }

    /// Subscribes to ticker events for the specified symbols and category.
    ///
    /// # Arguments
    ///
    /// * `subs` - A vector of symbols for which ticker events are subscribed.
    /// * `category` - The category for which ticker events are subscribed.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::Category;
    /// let subs = vec!["BTCUSD", "ETHUSD"];
    /// let category = Category::Linear;
    /// let sender = UnboundedSender<Ticker>;
    /// ws_tickers(subs, category, sender);
    /// ```
    pub async fn ws_tickers(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<Ticker>,
    ) -> Result<(), BybitError> {
        self.ws_tickers_internal(subs, category, sender, |ws_ticker: WsTicker| {
            Some(ws_ticker.data)
        })
        .await
    }

    /// Subscribes to ticker events with timestamp for the specified symbols and category.
    ///
    /// # Arguments
    ///
    /// * `subs` - A vector of symbols for which ticker events are subscribed.
    /// * `category` - The category for which ticker events are subscribed.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::Category;
    /// let subs = vec!["BTCUSD", "ETHUSD"];
    /// let category = Category::Linear;
    /// let sender = UnboundedSender<Ticker>;
    /// ws_timed_tickers(subs, category, sender);
    /// ```
    pub async fn ws_timed_tickers(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<Timed<Ticker>>,
    ) -> Result<(), BybitError> {
        self.ws_tickers_internal(subs, category, sender, |ticker: WsTicker| {
            Some(Timed {
                time: ticker.ts,
                data: ticker.data,
            })
        })
        .await
    }

    /// A high abstraction level stream of timed linear snapshots, which you can
    /// subscribe to using the receiver of the sender. Internally this method
    /// consumes the linear ticker API but instead of returning a stream of deltas
    /// we update the initial snapshot with all subsequent streams, and thanks
    /// to internally using `.scan` we you get `Timed<LinearTickerDataSnapshot>`,
    /// instead of `Timed<LinearTickerDataDelta>`.
    ///
    /// If you provide multiple symbols, the `LinearTickerDataSnapshot` values
    /// will be interleaved.
    ///
    /// # Usage
    /// ```no_run
    /// use bybit::prelude::*;
    /// use tokio::sync::mpsc;
    /// use std::sync::Arc;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///
    /// let ws: Arc<Stream> = Arc::new(Bybit::new(None, None));
    /// let (tx, mut rx) = mpsc::unbounded_channel::<Timed<LinearTickerDataSnapshot>>();
    /// tokio::spawn(async move {
    ///     ws.ws_timed_linear_tickers(vec!["BTCUSDT".to_owned(), "ETHUSDT".to_owned()], tx)
    ///         .await
    ///         .unwrap();
    /// });
    /// while let Some(ticker_snapshot) = rx.recv().await {
    ///     println!("{:#?}", ticker_snapshot);
    /// }
    /// }
    /// ```
    pub async fn ws_timed_linear_tickers(
        &self,
        subs: Vec<String>,
        sender: mpsc::UnboundedSender<Timed<LinearTickerDataSnapshot>>,
    ) -> Result<(), BybitError> {
        let (tx, mut rx) = mpsc::unbounded_channel::<Timed<LinearTickerData>>();
        // Spawn the WebSocket task
        let self_clone = self.clone();
        tokio::spawn(async move {
            self_clone
                .ws_tickers_internal(
                    subs.iter().map(|s| s.as_str()).collect(),
                    Category::Linear,
                    tx,
                    |ticker: WsTicker| match &ticker.data {
                        Ticker::Linear(linear) => Some(Timed {
                            time: ticker.ts,
                            data: linear.clone(),
                        }),
                        Ticker::Spot(_) => None,
                        Ticker::Options(_) => None,
                        Ticker::Futures(_) => None,
                    },
                )
                .await
        });

        // State to store snapshots for each symbol
        let mut snapshots: HashMap<String, Timed<LinearTickerDataSnapshot>> = HashMap::new();

        // Process incoming messages
        while let Some(ticker) = rx.recv().await {
            match ticker.data {
                LinearTickerData::Snapshot(snapshot) => {
                    let symbol = snapshot.symbol.clone();
                    let timed_snapshot = Timed {
                        time: ticker.time,
                        data: snapshot,
                    };
                    // Store the snapshot and send it
                    snapshots.insert(symbol.clone(), timed_snapshot.clone());
                    send_or_err(&sender, timed_snapshot)?
                }
                LinearTickerData::Delta(delta) => {
                    let symbol = delta.symbol.clone();
                    if let Some(snapshot_timed) = snapshots.get_mut(&symbol) {
                        let mut snapshot = snapshot_timed.data.clone();
                        snapshot.update(delta);
                        let new = Timed {
                            data: snapshot,
                            time: ticker.time,
                        };
                        *snapshot_timed = new.clone();
                        send_or_err(&sender, new)?
                    }
                    // If no snapshot exists for the symbol, skip the delta
                }
            }
        }

        Ok(())
    }

    async fn ws_tickers_internal<T, F>(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<T>,
        filter_map: F,
    ) -> Result<(), BybitError>
    where
        T: 'static + Sync + Send,
        F: 'static + Sync + Send + Fn(WsTicker) -> Option<T>,
    {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|sub| format!("tickers.{}", sub.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(String::as_str).collect());

        self.subscribe_channel(request, category, sender, move |event| {
            if let WebsocketEvents::TickerEvent(ticker) = event {
                filter_map(ticker)
            } else {
                None
            }
        })
        .await
    }

    pub async fn ws_liquidations(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<LiquidationData>,
    ) -> Result<(), BybitError> {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|sub| format!("allLiquidation.{}", sub.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(String::as_str).collect());

        self.subscribe_channel(request, category, sender, |event| {
            if let WebsocketEvents::LiquidationEvent(liquidation) = event {
                Some(liquidation.data)
            } else {
                None
            }
        })
        .await
    }

    pub async fn ws_klines(
        &self,
        subs: Vec<(&str, &str)>,
        category: Category,
        sender: mpsc::UnboundedSender<WsKline>,
    ) -> Result<(), BybitError> {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|(interval, sym)| format!("kline.{}.{}", interval, sym.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(AsRef::as_ref).collect());
        self.subscribe_channel(request, category, sender, |event| {
            if let WebsocketEvents::KlineEvent(kline) = event {
                Some(kline)
            } else {
                None
            }
        })
        .await
    }

    pub async fn ws_position(
        &self,
        cat: Option<Category>,
        sender: mpsc::UnboundedSender<PositionData>,
    ) -> Result<(), BybitError> {
        let sub_str = if let Some(v) = cat {
            match v {
                Category::Linear => "position.linear",
                Category::Inverse => "position.inverse",
                _ => "",
            }
        } else {
            "position"
        };

        let request = Subscription::new("subscribe", vec![sub_str]);
        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::PositionEvent(position) = event {
                for v in position.data {
                    send_or_err(&sender, v)?;
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn ws_executions(
        &self,
        cat: Option<Category>,
        sender: mpsc::UnboundedSender<ExecutionData>,
    ) -> Result<(), BybitError> {
        let sub_str = if let Some(v) = cat {
            match v {
                Category::Linear => "execution.linear",
                Category::Inverse => "execution.inverse",
                Category::Spot => "execution.spot",
                Category::Option => "execution.option",
            }
        } else {
            "execution"
        };

        let request = Subscription::new("subscribe", vec![sub_str]);
        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::ExecutionEvent(execute) = event {
                for v in execute.data {
                    send_or_err(&sender, v)?;
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn ws_fast_exec(
        &self,
        sender: mpsc::UnboundedSender<FastExecData>,
    ) -> Result<(), BybitError> {
        let sub_str = "execution.fast";
        let request = Subscription::new("subscribe", vec![sub_str]);

        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::FastExecEvent(execution) = event {
                for v in execution.data {
                    send_or_err(&sender, v)?;
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn ws_orders(
        &self,
        cat: Option<Category>,
        sender: mpsc::UnboundedSender<OrderData>,
    ) -> Result<(), BybitError> {
        let sub_str = if let Some(v) = cat {
            match v {
                Category::Linear => "order.linear",
                Category::Inverse => "order.inverse",
                Category::Spot => "order.spot",
                Category::Option => "order.option",
            }
        } else {
            "order"
        };

        let request = Subscription::new("subscribe", vec![sub_str]);
        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::OrderEvent(order) = event {
                for v in order.data {
                    send_or_err(&sender, v)?;
                }
            }
            Ok(())
        })
        .await
    }

    pub async fn ws_wallet(
        &self,
        sender: mpsc::UnboundedSender<WalletData>,
    ) -> Result<(), BybitError> {
        let sub_str = "wallet";
        let request = Subscription::new("subscribe", vec![sub_str]);
        self.ws_priv_subscribe(request, move |event| {
            if let WebsocketEvents::Wallet(wallet) = event {
                for v in wallet.data {
                    send_or_err(&sender, v)?;
                }
            }
            Ok(())
        })
        .await
    }

    /// Subscribes to system status updates via WebSocket.
    ///
    /// System status updates provide real-time information about platform maintenance
    /// or service incidents. This is useful for monitoring exchange health and
    /// planning trading activities around maintenance windows.
    ///
    /// # Arguments
    ///
    /// * `sender` - Channel sender for system status updates
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if subscription succeeds, otherwise returns a `BybitError`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rs_bybit::prelude::*;
    /// use tokio::sync::mpsc;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), BybitError> {
    /// let client = Client::new("api_key", "api_secret", None, None)?;
    /// let stream = Stream { client };
    /// let (tx, mut rx) = mpsc::unbounded_channel();
    ///
    /// stream.ws_system_status(tx).await?;
    ///
    /// while let Some(update) = rx.recv().await {
    ///     println!("System status update: {:?}", update);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ws_system_status(
        &self,
        sender: mpsc::UnboundedSender<SystemStatusUpdate>,
    ) -> Result<(), BybitError> {
        let request = Subscription::new("subscribe", vec!["system.status"]);
        let request_str = Self::build_subscription(request);

        // System status uses the misc/status endpoint
        let endpoint = WebsocketAPI::PublicMiscStatus;
        let response = self
            .client
            .wss_connect(endpoint, Some(request_str), false, None)
            .await?;

        let handler = move |event| {
            if let WebsocketEvents::SystemStatusEvent(status_update) = event {
                send_or_err(&sender, status_update)?;
            }
            Ok(())
        };

        let mut ws_client = WsClient::new(response);
        Self::event_loop(&mut ws_client, handler, None).await?;
        Ok(())
    }

    pub async fn ws_trade_stream<'a, F>(
        &self,
        req: mpsc::UnboundedReceiver<RequestType<'a>>,
        handler: F,
    ) -> Result<(), BybitError>
    where
        F: FnMut(WebsocketEvents) -> Result<(), BybitError> + 'static + Send,
        'a: 'static,
    {
        let response = self
            .client
            .wss_connect(WebsocketAPI::TradeStream, None, true, Some(10))
            .await?;
        let mut ws_client = WsClient::new(response);
        Self::event_loop(&mut ws_client, handler, Some(req)).await?;

        Ok(())
    }

    pub async fn event_loop<'a, H>(
        client: &mut WsClient,
        mut handler: H,
        mut request_sender: Option<mpsc::UnboundedReceiver<RequestType<'_>>>,
    ) -> Result<(), BybitError>
    where
        H: WebSocketHandler,
    {
        let mut interval = Instant::now();
        loop {
            let msg = client.stream().next().await;
            match msg {
                Some(Ok(WsMessage::Text(msg))) => {
                    handler.handle_msg(&msg)?;
                }
                Some(Ok(WsMessage::Ping(data))) => {
                    let _ = client.stream().send(WsMessage::Pong(data)).await;
                }
                Some(Ok(WsMessage::Pong(_))) => {
                    // Protocol-level pong received, no action needed
                }
                Some(Err(e)) => {
                    return Err(BybitError::from(e.to_string()));
                }
                None => {
                    return Err(BybitError::Base("Stream was closed".to_string()));
                }
                _ => {}
            }
            if let Some(sender) = request_sender.as_mut() {
                match sender.try_recv() {
                    Ok(v) => match v {
                        RequestType::Subscribe(sub) | RequestType::Unsubscribe(sub) => {
                            let req = Self::build_subscription(sub);
                            let _ = client
                                .stream()
                                .send(WsMessage::Text(req.into()))
                                .await
                                .map_err(BybitError::from);
                        }
                        _ => {
                            if let Ok(req) = Self::build_trade_subscription(v, Some(3000)) {
                                let _ = client
                                    .stream()
                                    .send(WsMessage::Text(req.into()))
                                    .await
                                    .map_err(BybitError::from);
                            }
                        }
                    },
                    Err(mpsc::error::TryRecvError::Empty) => {}
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        request_sender = None;
                    }
                }
            }

            if interval.elapsed() > PING_INTERVAL {
                let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
                parameters.insert("op".into(), "ping".into());
                let request = build_json_request(&parameters);
                let _ = client
                    .stream()
                    .send(WsMessage::Text(request.into()))
                    .await
                    .map_err(BybitError::from);
                interval = Instant::now();
            }
        }
    }

    /// Gracefully disconnects the WebSocket client.
    pub async fn disconnect(&self, client: &mut WsClient) -> Result<(), BybitError> {
        client.disconnect().await
    }
}

pub trait WebSocketHandler {
    type Event;
    fn handle_msg(&mut self, msg: &str) -> Result<(), BybitError>;
}

impl<F> WebSocketHandler for F
where
    F: FnMut(WebsocketEvents) -> Result<(), BybitError>,
{
    type Event = WebsocketEvents;
    fn handle_msg(&mut self, msg: &str) -> Result<(), BybitError> {
        let update: Value = serde_json::from_str(msg)?;
        if let Ok(event) = serde_json::from_value::<WebsocketEvents>(update.clone()) {
            self(event)?;
        }

        Ok(())
    }
}
