use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::Write;
use tokio_tungstenite::connect_async;
use url::Url;
use crate::order_book::OrderBook;
use crate::exchange::Position;
use crate::momentum::{MomentumStrategy, Action};

#[derive(Debug, Deserialize)]
struct CoinbaseTrade {
    #[serde(rename = "price")]
    price: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
struct CoinbaseMessage {
    #[serde(flatten)]
    trade: Option<CoinbaseTrade>,
}

pub async fn stream_coinbase_prices(mut tx: tokio::sync::mpsc::Sender<f64>) {
    let url = Url::parse("wss://ws-feed.exchange.coinbase.com").unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to Coinbase WebSocket");
    let (mut write, mut read) = ws_stream.split();

    // Subscribe to BTC-USD match channel
    let subscribe_msg = serde_json::json!({
        "type": "subscribe",
        "channels": [{
            "name": "matches",
            "product_ids": ["BTC-USD"]
        }]
    });

    let _ = write
        .send(tokio_tungstenite::tungstenite::Message::Text(subscribe_msg.to_string()))
        .await;

    let mut ob = OrderBook::new();
    let mut strat = MomentumStrategy::new(5, 0.05);
    let mut position = Position::new();
    let quantity = 1.0;
    let spread = 0.10;

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("trades.csv")
        .expect("Cannot open trades.csv");
    writeln!(log_file, "action,price,quantity,realized_pnl,unrealized_pnl").ok();

    while let Some(Ok(msg)) = read.next().await {
        if let Ok(text) = msg.to_text() {
            if let Ok(parsed) = serde_json::from_str::<CoinbaseMessage>(text) {
                if let Some(trade) = parsed.trade {
                    if let Ok(mid_price) = trade.price.parse::<f64>() {
                        let bid = mid_price - spread / 2.0;
                        let ask = mid_price + spread / 2.0;

                        ob.bids.clear();
                        ob.asks.clear();
                        ob.update_bid(bid, 10.0);
                        ob.update_ask(ask, 5.0);

                        let action = strat.update(bid);
                        let exec_price = match action {
                            Action::Buy => ask,
                            Action::Sell => bid,
                            Action::Hold => bid,
                        };

                        if let Some(trade) = position.execute(action.clone(), exec_price, quantity) {
                            println!("Executed {:?} at ${:.2} for {:.0} units", trade.action, trade.price, trade.quantity);
                            writeln!(
                                log_file,
                                "{:?},{:.2},{:.2},{:.2},{:.2}",
                                trade.action,
                                trade.price,
                                trade.quantity,
                                position.realized_pnl,
                                position.unrealized_pnl(bid)
                            ).ok();
                        } else if matches!(action, Action::Sell) {
                            println!("Tried to SELL but insufficient quantity.");
                        }

                        println!(
                            "Position: {:.1} units @ avg ${:.2} | Realized P&L: ${:.2} | Unrealized P&L: ${:.2}",
                            position.quantity,
                            position.avg_cost,
                            position.realized_pnl,
                            position.unrealized_pnl(bid),
                        );

                        let _ = tx.send(mid_price).await;
                    }
                }
            }
        }
    }
}
