use dotenv::dotenv;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

type HmacSha256 = Hmac<Sha256>;

fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY not set.");
    let api_secret = env::var("API_SECRET").expect("API_SECRET not set.");

    let mut mac: HmacSha256 =
        HmacSha256::new_from_slice(api_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(api_key.as_bytes());
    let signature = mac.finalize();

    const BASE_URL: &str = "wss://stream.binance.com:9443/ws";
    const INDIV_SYM_TICK: &str = "BTC@miniTicker";

    println!("Hello, world!");
}
