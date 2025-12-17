use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

// Connects to Binance trade stream and prints incoming raw messages
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let _ = rustls::crypto::ring::default_provider().install_default();
	
	let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";
	println!("Connecting to {url} ...");

	let (ws_stream, _) = connect_async(url).await?;
	println!("Connected. Waiting for trades ...");

	let (mut write, mut read) = ws_stream.split();

	while let Some(msg) = read.next().await {
		match msg? {
			Message::Text(text) => println!("trade: {text}"),
			
			Message::Binary(bin) => println!("trade (binary): {:?}", bin),
			Message::Ping(payload) => {
				write.send(Message::Pong(payload)).await.ok();
			}
			Message::Close(frame) => {
				println!("server closed connection: {:?}", frame);
				break;
			}
			_ => (),
		}
	}

	Ok(())
}
