use std::error::Error;
use std::time::Duration;

use ws::{connect, CloseCode};
use serde::{Deserialize, Serialize};
use serde_json::Result;



#[derive(Deserialize, Serialize, Debug)]
struct Message {
    #[serde(rename = "type")]
    message_type: String,
    payload: Payload,
}

#[derive(Deserialize, Serialize, Debug)]
struct Payload {
    block: Block,
}

#[derive(Deserialize, Serialize, Debug)]
struct Block {
    number: u64,
    transactions: Vec<Transaction>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Transaction {
    hash: String,
    to: String,
    value: String,
}


fn listen_for_messages() -> Result<(), Box<dyn Error>> {
    connect("wss://api.blocknative.com/v1/ws", |out| {
        out.on_text(|message| {
            let message: Message = serde_json::from_str(&message)?;
            println!("Received message: {:?}", message);
            Ok(())
        });

        out.on_close(|code, reason| {
            println!("Connection closed: {} {}", code, reason);
        });

        out.on_open(|| {
            println!("Connected to Blocknative WebSocket API");
            out.send("{\"type\": \"subscribe\", \"payload\": {\"event\": \"all\"}}")
        });

        out.on_shutdown(|| {
            println!("Shutting down");
        });

        Ok(())
    })
        .expect("Failed to connect to Blocknative WebSocket API")
        .recv_timeout(Duration::from_secs(60))
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Uniswap arbitrage bot...");

    listen_for_messages()?;

    println!("Uniswap arbitrage bot stopped");

    Ok(())
}
