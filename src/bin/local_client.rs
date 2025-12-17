use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

// 同じPC内でサーバーに接続する簡単なクライアント
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:9001";
    println!("Connecting to {url} ...");

    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to local server.");

    let (mut write, mut read) = ws_stream.split();

    // 最初のメッセージを送信（String -> Utf8Bytes へ変換）
    write
        .send(Message::Text("こんにちは、1回目のローカルサーバー!".to_string().into()))
        .await?;

    write
        .send(Message::Text("2回目のメッセージです。".to_string().into()))
        .await?;

    // サーバーからの1メッセージを受け取って表示して終了
    while let Some(msg) = read.next().await {
		match msg? {
			Message::Text(text) => println!("受け取ったテキスト: {text}"),
			
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
