mod order_book;
mod strategy;
mod exchange;
mod market;
mod momentum;

use dotenvy::dotenv;
use std::env;
use market::get_trade_price;
use reqwest::Client;
use exchange::{Position};
use momentum::{MomentumStrategy, Action};
use order_book::OrderBook;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("ALPACA_API_KEY").expect("Missing API key");
    let secret = env::var("ALPACA_SECRET_KEY").expect("Missing secret");
    let client = Client::new();

    let mut ob = OrderBook::new();
    let mut strat = MomentumStrategy::new(2, 0.001);
    let mut position = Position::new();
    let quantity = 1.0;

    for _ in 0..10 {
        if let Some((price)) = get_trade_price("AAPL", &client, &api_key, &secret).await {
            ob.bids.clear();
            ob.asks.clear();
            ob.update_bid(price, 10.0);
            ob.update_ask(price + 0.01, 5.0);

            let action = strat.update(price);
            println!("Strategy decision: {:?}", action);

            if let Some(trade) = position.execute(action.clone(), price, quantity) {
                println!(
                    "Executed {:?} at ${:.2} for {:.0} units",
                    trade.action, trade.price, trade.quantity
                );
            } else if matches!(action, Action::Sell) {
                println!("Tried to SELL but insufficient quantity.");
            }

            println!(
                "Position: {:.1} units @ avg ${:.2} | Realized P&L: ${:.2} | Unrealized P&L: ${:.2}",
                position.quantity,
                position.avg_cost,
                position.realized_pnl,
                position.unrealized_pnl(price),
            );
        } else {
            println!("Failed to fetch quote.");
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
