use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;

// 同じPC内での簡単なローカル WebSocket サーバー
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 127.0.0.1:9001 で待ち受け
    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    println!("Server listening on ws://127.0.0.1:9001");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("新しい接続: {addr}");

        tokio::spawn(async move {
            // WebSocket ハンドシェイク
            let ws_stream = accept_async(stream).await.expect("WebSocket accept failed");
            println!("WebSocket 接続確立: {addr}");

            let (mut write, mut read) = ws_stream.split();

            // クライアントからのメッセージを受信して、そのままエコーする
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        println!("from {addr}: {text}");
                        if let Err(e) = write
                            .send(Message::Text(format!("echo: {text}").into()))
                            .await
                        {
                            eprintln!("send error: {e}");
                            break;
                        }
                    }
                    Ok(Message::Close(frame)) => {
                        println!("closed by client: {:?}", frame);
                        break;
                    }
                    Ok(other) => {
                        println!("other message: {:?}", other);
                    }
                    Err(e) => {
                        eprintln!("receive error: {e}");
                        break;
                    }
                }
            }
        });
    }
}
