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

    // 最初のメッセージを送信
    write
        .send(Message::Text("こんにちは、ローカルサーバー!".to_string()))
        .await?;

    // サーバーからの1メッセージを受け取って表示して終了
    if let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => println!("from server: {text}"),
            other => println!("other message from server: {:?}", other),
        }
    }

    Ok(())
}
