mod order_book;
mod strategy;
mod exchange;

fn main() {
    let mut ob = order_book::OrderBook::new();
    ob.update_bid(100.5, 10.0);
    ob.update_ask(101.0, 5.0);
    ob.update_bid(100.0, 7.0);
    ob.update_ask(101.5, 2.5);

    println!("Bids:");
    for (price, size) in &ob.bids {
        println!("Price: {}, Size: {}", price, size);
    }

    println!("Asks:");
    for (price, size) in &ob.asks {
        println!("Price: {}, Size: {}", price, size);
    }
}

