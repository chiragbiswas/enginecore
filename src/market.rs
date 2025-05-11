use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Trade {
    p: f64, // last trade price
}

#[derive(Debug, Deserialize)]
struct TradeResponse {
    trade: Trade,
}

pub async fn get_trade_price(
    symbol: &str,
    client: &Client,
    api_key: &str,
    secret: &str,
) -> Option<f64> {
    let url = format!(
        "https://data.alpaca.markets/v2/stocks/{}/trades/latest",
        symbol
    );

    let res = client
        .get(&url)
        .header("APCA-API-KEY-ID", api_key)
        .header("APCA-API-SECRET-KEY", secret)
        .send()
        .await
        .ok()?;

    let json: TradeResponse = res.json().await.ok()?;
    Some(json.trade.p)
}


