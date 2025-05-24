use bybit::prelude::*;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::test;
    use tokio::{sync::mpsc, time::Instant};

    use super::*;

    static API_KEY: &str = ""; //Mockup string
    static SECRET: &str = ""; // Mockup string

    #[test]
    async fn test_auth() {
        let ws: Stream = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let (tx, mut rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            ws.ws_wallet(tx).await.unwrap();
        });
        while let Some(data) = rx.recv().await {
            println!("{:#?}", data);
        }
    }

    #[test]
    async fn ping() {
        let ws: Stream = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let response = ws.ws_ping(true).await;
        println!("{:#?}", response);
    }

    #[test]
    async fn test_order_book() {
        let ws: Stream = Bybit::new(None, None);
        let request = Subscription {
            args: vec!["publicTrade.ETHUSDT"],
            op: "subscribe",
        };

        let response = ws
            .ws_subscribe(request, Category::Linear, |event| {
                match event {
                    WebsocketEvents::TradeEvent(trade) => {
                        // Handle Trade
                        for v in trade.data {
                            println!(
                                "Volume: {:.3} USD, Timestamp: {}, Side: {} Time:{}",
                                v.volume * v.price,
                                v.timestamp / 6000,
                                v.side,
                                Instant::now().elapsed().as_nanos()
                            );
                        }
                    }
                    WebsocketEvents::OrderBookEvent(order_book) => {
                        println!("{:#?}", order_book.data);
                        // Handle OrderBook event
                    }
                    // Add additional matches for other variants of the WebsocketEvents enum
                    WebsocketEvents::TickerEvent(ticker) => {
                        // Handle Ticker event
                        match ticker.data {
                            Ticker::Linear(linear_ticker) => {
                                println!("{:#?}", linear_ticker);
                            }
                            Ticker::Spot(spot_ticker) => {
                                println!("{:#?}", spot_ticker);
                            }
                        }
                    }
                    WebsocketEvents::KlineEvent(kline) => {
                        // Handle Kline
                        for v in kline.data {
                            println!("{:#?}", v);
                        }
                    }
                    WebsocketEvents::LiquidationEvent(liquidation) => {
                        // Handle Liquidation
                        println!("{:#?}", liquidation.data);
                    }
                    _ => {}
                };
                Ok(())
            })
            .await;
        println!("{:#?}", response);
    }

    #[test]
    async fn test_default_orderbook() {
        let ws: Stream = Bybit::new(None, None);
        let (tx, mut rx) = mpsc::unbounded_channel();
        let request = vec![(1, "ETHUSDT")];
        tokio::spawn(async move {
            ws.ws_orderbook(request, Category::Linear, tx)
                .await
                .unwrap();
        });
        while let Some(data) = rx.recv().await {
            println!("{:#?}", data);
        }
    }

    #[test]
    async fn test_ws_snapshot_scan() {
        let ws: Arc<Stream> = Arc::new(Bybit::new(None, None));
        let (tx, mut rx) = mpsc::unbounded_channel::<Timed<LinearTickerDataSnapshot>>();
        tokio::spawn(async move {
            ws.ws_timed_linear_tickers(vec!["BTCUSDT".to_owned()], tx)
                .await
                .unwrap();
        });
        while let Some(ticker_snapshot) = rx.recv().await {
            println!("{:#?}", ticker_snapshot);
        }
    }

    #[test]
    async fn test_default_trades() {
        let ws: Stream = Bybit::new(None, None);
        let request = vec!["BTCUSDT", "SOLUSDT", "ETHUSDT", "XRPUSDT"];
        let (tx, mut rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            ws.ws_trades(request, Category::Linear, tx).await.unwrap();
        });
        while let Some(data) = rx.recv().await {
            println!("{:#?}", data);
        }
    }

    #[test]
    async fn test_default_tickers() {
        let ws: Stream = Bybit::new(None, None);
        let request = vec!["ETHUSDT", "SOLUSDT"];
        let (tx, mut rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            ws.ws_tickers(request, Category::Linear, tx).await.unwrap();
        });
        while let Some(data) = rx.recv().await {
            match data {
                Ticker::Linear(linear_ticker) => {
                    println!("{:#?}", linear_ticker);
                }
                Ticker::Spot(spot_ticker) => {
                    println!("{:#?}", spot_ticker);
                }
            }
        }
    }

    #[test]
    async fn test_default_klines() {
        let ws: Stream = Bybit::new(None, None);
        let request = vec![("1", "ETHUSDT")];
        let (tx, mut rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            ws.ws_klines(request, Category::Linear, tx).await.unwrap();
        });
        while let Some(data) = rx.recv().await {
            println!("{:#?}", data);
        }
    }
}
