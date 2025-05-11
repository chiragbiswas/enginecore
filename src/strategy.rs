use crate::order_book::OrderBook;
use ordered_float::OrderedFloat;



pub enum Action {
    Buy,
    Sell,
    Hold,
}


pub fn simple_strategy(ob: &OrderBook) -> Action {
    if let Some((best_bid, _)) = ob.bids.iter().rev().next() {
        if best_bid.0 > 100.0 {
            return Action::Buy;
        }
    }
    Action::Hold
}
