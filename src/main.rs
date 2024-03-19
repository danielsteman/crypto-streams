use dotenv::dotenv;
use hex;
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use std::env;
use tungstenite::{connect, Message};
use url::Url;

type HmacSha256 = Hmac<Sha256>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let api_secret = env::var("SECRET_KEY").expect("SECRET_KEY not set.");

    let mut mac: HmacSha256 =
        HmacSha256::new_from_slice(api_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(api_key.as_bytes());
    let signature = mac.finalize().into_bytes();

    let hex_signature = hex::encode(signature);

    println!("Signature: {}", hex_signature);

    let example_requests = json!({
      "id": "e2a85d9f-07a5-4f94-8d5f-789dc3deb097",
      "method": "order.place",
      "params": {
        "symbol": "BTCUSDT",
        "side": "BUY",
        "type": "LIMIT",
        "price": "0.1",
        "quantity": "10",
        "timeInForce": "GTC",
        "timestamp": 1655716096498_i64,
        "apiKey": "T59MTDLWlpRW16JVeZ2Nju5A5C98WkMm8CSzWC4oqynUlTm1zXOxyauT8LmwXEv9",
        "signature": "5942ad337e6779f2f4c62cd1c26dba71c91514400a24990a3e7f5edec9323f90"
      }
    });

    let (mut socket, response) =
        connect(Url::parse("wss://testnet.binance.vision/ws-api/v3").unwrap())
            .expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response body: {:?}", response);
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }
}
