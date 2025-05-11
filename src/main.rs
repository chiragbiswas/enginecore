mod order_book;
mod strategy;
mod exchange;
mod market;
mod momentum;

use exchange::{Position, Trade};
use momentum::{MomentumStrategy, Action};
use market::Market;
use std::{thread, time::Duration};
fn main() {
    let mut ob = order_book::OrderBook::new();
    let mut market = Market::new(100.0);
    let mut strat = MomentumStrategy::new(10, 0.3);
    let mut position = Position::new();

    for _ in 0..1000 {
        let (bid, ask) = market.next_tick();
        ob.bids.clear();
        ob.asks.clear();
   
        ob.update_bid(bid, 10.0);
        ob.update_ask(ask, 5.0);

        let action = strat.update(bid);
        let quantity = 1.0;
        if let Some(trade) = position.execute(action.clone(), bid, quantity)
 {
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
            position.unrealized_pnl(bid),
        );
    


        thread::sleep(Duration::from_millis(500)); //simulate time passing
    }
}
