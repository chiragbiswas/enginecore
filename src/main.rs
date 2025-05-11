mod order_book;
mod exchange;
mod momentum;
mod coinbase_feed;

use tokio::sync::mpsc;
use coinbase_feed::stream_coinbase_prices;

#[tokio::main]
async fn main() {
    let (tx, _rx) = mpsc::channel(100);
    tokio::spawn(async move {
        stream_coinbase_prices(tx).await;
    });

    // Optional: keep main alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

