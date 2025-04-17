// src/data_collector.rs

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};

/// Represents a cleaned and usable piece of trade data
#[derive(Debug, Clone)]
pub struct MarketData {
    pub price: f64,
    pub volume: f64,
    pub side: String, // "b" = buy, "s" = sell
}

pub async fn connect_to_kraken() {
    //Connect to kraken public websocket API
    let url = "wss://ws.kraken.com/";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to Kraken WebSocket API");
    println!("Connected to Kraken WebSocket API");

    //split the stream into wrriter (for sending) and reader (for receiving)
    let (mut write, mut read) = ws_stream.split();

    //Create ssubscrioption message as a JSON value
    let subscribe_message = serde_json::json!({
        "event": "subscribe",
        "pair": ["BTC/USD"],
        "subscription": {
            "name": "trade"
        }
    });

    //Send the subscription message to the server
    write.send(Message::Text(subscribe_message.to_string())).await.expect("Failed to send subscription message");
    println!("[DataCollector] Subscribed to BTC/USD trade feed");

    //Read and print incoming message
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            println!("[Kraken] {}", text);
        }
    }
}