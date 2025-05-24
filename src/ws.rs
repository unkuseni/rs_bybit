use crate::prelude::*;

use async_stream::stream;
use futures::{SinkExt, StreamExt};
use log::trace;
use std::future::ready;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{tungstenite::Message as WsMessage, MaybeTlsStream};

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
        if let Ok(_) = Self::event_loop(response, handler, None).await {}
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
        let endpoint = {
            match category {
                Category::Linear => WebsocketAPI::PublicLinear,
                Category::Inverse => WebsocketAPI::PublicInverse,
                Category::Spot => WebsocketAPI::PublicSpot,
                _ => unimplemented!("Option has not been implemented"),
            }
        };
        let request = Self::build_subscription(req);
        let response = self
            .client
            .wss_connect(endpoint, Some(request), false, None)
            .await?;
        Self::event_loop(response, handler, None).await?;
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

    pub fn build_trade_subscription(orders: RequestType, recv_window: Option<u64>) -> String {
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
        }
        build_json_request(&parameters)
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
        self.ws_subscribe(request, category, move |event| {
            if let WebsocketEvents::OrderBookEvent(order_book) = event {
                sender
                    .send(order_book)
                    .map_err(|e| BybitError::ChannelSendError {
                        underlying: e.to_string(),
                    })?;
            }
            Ok(())
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
                    sender
                        .send(trade)
                        .map_err(|e| BybitError::ChannelSendError {
                            underlying: e.to_string(),
                        })?;
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
        self._ws_tickers(subs, category, sender, |ws_ticker| Some(ws_ticker.data))
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
        self._ws_tickers(subs, category, sender, |ticker| {
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
    /// # Usage
    /// ```
    /// let ws: Arc<Stream> = Arc::new(Bybit::new(None, None));
    /// let (tx, mut rx) = mpsc::unbounded_channel::<Timed<LinearTickerDataSnapshot>>();
    /// tokio::spawn(async move {
    ///     ws.ws_timed_linear_tickers(vec!["BTCUSDT".to_owned()], tx)
    ///         .await
    ///         .unwrap();
    /// });
    /// while let Some(ticker_snapshot) = rx.recv().await {
    ///     println!("{:#?}", ticker_snapshot);
    /// }
    /// ```
    pub async fn ws_timed_linear_tickers(
        self: Arc<Self>,
        subs: Vec<String>,
        sender: mpsc::UnboundedSender<Timed<LinearTickerDataSnapshot>>,
    ) -> Result<(), BybitError> {
        let (tx, mut rx) = mpsc::unbounded_channel::<Timed<LinearTickerData>>();

        // Spawn the WebSocket task
        tokio::spawn({
            let self_arc = Arc::clone(&self);
            let subs = subs.clone();
            async move {
                self_arc
                    ._ws_tickers(
                        subs.iter().map(|s| s.as_str()).collect(),
                        Category::Linear,
                        tx,
                        |ticker| -> Option<Timed<LinearTickerData>> {
                            match &ticker.data {
                                Ticker::Linear(linear) => Some(Timed {
                                    time: ticker.ts,
                                    data: linear.clone(),
                                }),
                                Ticker::Spot(_) => None,
                            }
                        },
                    )
                    .await
            }
        });

        // Get the first snapshot
        let initial: Timed<LinearTickerDataSnapshot> = match rx.recv().await {
            Some(ticker) => match ticker.data {
                LinearTickerData::Snapshot(snapshot) => Timed {
                    time: ticker.time,
                    data: snapshot,
                },
                LinearTickerData::Delta(_) => {
                    return Err(BybitError::Base(
                        "First message was not a snapshot".to_owned(),
                    ))
                }
            },
            None => {
                return Err(BybitError::Base(
                    "Stream ended before receiving snapshot".to_owned(),
                ))
            }
        };

        // Send the initial snapshot
        if sender.send(initial.clone()).is_err() {
            return Err(BybitError::Base(
                "Failed to send initial snapshot".to_owned(),
            ));
        }

        // Create a stream for deltas
        let delta_stream = stream! {
            while let Some(ticker) = rx.recv().await {
                match ticker.data {
                    LinearTickerData::Snapshot(_) => {
                        // Ignore unexpected snapshots
                        continue;
                    }
                    LinearTickerData::Delta(delta) => yield Timed {
                        time: ticker.time,
                        data: delta,
                    },
                }
            }
        };

        // Pin the stream for scanning
        let delta_stream = Box::pin(delta_stream);

        // Use scan to update the snapshot with deltas
        let result = delta_stream
            .scan(initial, |acc, next| {
                let mut acc_ticker = acc.data.clone();
                acc_ticker.update(next.data);
                let new = Timed {
                    data: acc_ticker,
                    time: next.time,
                };
                *acc = new.clone();
                ready(Some(new))
            })
            .for_each(|item| async {
                let _ = sender.send(item); // Ignore send errors to continue processing
            })
            .await;

        Ok(result)
    }

    async fn _ws_tickers<T, F>(
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

        let handler = move |event| {
            if let WebsocketEvents::TickerEvent(ticker) = event {
                if let Some(mapped) = filter_map(ticker) {
                    sender
                        .send(mapped)
                        .map_err(|e| BybitError::ChannelSendError {
                            underlying: e.to_string(),
                        })?;
                }
            }
            Ok(())
        };

        self.ws_subscribe(request, category, handler).await
    }
    pub async fn ws_liquidations(
        &self,
        subs: Vec<&str>,
        category: Category,
        sender: mpsc::UnboundedSender<LiquidationData>,
    ) -> Result<(), BybitError> {
        let arr: Vec<String> = subs
            .into_iter()
            .map(|sub| format!("liquidation.{}", sub.to_uppercase()))
            .collect();
        let request = Subscription::new("subscribe", arr.iter().map(String::as_str).collect());

        let handler = move |event| {
            if let WebsocketEvents::LiquidationEvent(liquidation) = event {
                sender
                    .send(liquidation.data)
                    .map_err(|e| BybitError::ChannelSendError {
                        underlying: e.to_string(),
                    })?;
            }
            Ok(())
        };

        self.ws_subscribe(request, category, handler).await
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
        self.ws_subscribe(request, category, move |event| {
            if let WebsocketEvents::KlineEvent(kline) = event {
                sender
                    .send(kline)
                    .map_err(|e| BybitError::ChannelSendError {
                        underlying: e.to_string(),
                    })?;
            }
            Ok(())
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
                    sender.send(v).map_err(|e| BybitError::ChannelSendError {
                        underlying: e.to_string(),
                    })?;
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
                    sender.send(v).map_err(|e| BybitError::ChannelSendError {
                        underlying: e.to_string(),
                    })?;
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
                    sender.send(v).map_err(|e| BybitError::ChannelSendError {
                        underlying: e.to_string(),
                    })?;
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
                    sender.send(v).map_err(|e| BybitError::ChannelSendError {
                        underlying: e.to_string(),
                    })?;
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
                    sender.send(v).map_err(|e| BybitError::ChannelSendError {
                        underlying: e.to_string(),
                    })?;
                }
            }
            Ok(())
        })
        .await
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
        Self::event_loop(response, handler, Some(req)).await?;

        Ok(())
    }

    pub async fn event_loop<'a, H>(
        mut stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
        mut handler: H,
        mut order_sender: Option<mpsc::UnboundedReceiver<RequestType<'_>>>,
    ) -> Result<(), BybitError>
    where
        H: WebSocketHandler,
    {
        let mut interval = Instant::now();
        loop {
            let msg = stream.next().await;
            match msg {
                Some(Ok(WsMessage::Text(msg))) => {
                    if let Err(_) = handler.handle_msg(&msg) {
                        return Err(BybitError::Base(
                            "Error handling stream message".to_string(),
                        ));
                    }
                }
                Some(Err(e)) => {
                    return Err(BybitError::from(e.to_string()));
                }
                None => {
                    return Err(BybitError::Base("Stream was closed".to_string()));
                }
                _ => {}
            }
            if let Some(sender) = order_sender.as_mut() {
                if let Some(v) = sender.recv().await {
                    let order_req = Self::build_trade_subscription(v, Some(3000));
                    stream.send(WsMessage::Text(order_req)).await?;
                }
            }

            if interval.elapsed() > Duration::from_secs(300) {
                let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
                if order_sender.is_none() {
                    parameters.insert("req_id".into(), generate_random_uid(8).into());
                }
                parameters.insert("op".into(), "ping".into());
                let request = build_json_request(&parameters);
                let _ = stream
                    .send(WsMessage::Text(request))
                    .await
                    .map_err(BybitError::from);
                interval = Instant::now();
            }
        }
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
