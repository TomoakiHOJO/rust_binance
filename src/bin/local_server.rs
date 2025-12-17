use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};
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
            let mut s = 0;

            // クライアントからのメッセージを受信して処理する
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

                        // Ping を送って、クライアント側の Pong 応答をテストする
                        if let Err(e) = write
                            .send(Message::Ping(b"ping from server".to_vec().into()))
                            .await
                        {
                            eprintln!("send ping error: {e}");
                            break;
                        }
                        s += 1;
                        if s >= 2 {
                            // break する前に少し待つ（Delay）
                            sleep(Duration::from_secs(3)).await;

                            // 2回メッセージを処理したらサーバー側から Close を送る
                            if let Err(e) = write.send(Message::Close(None)).await {
                                eprintln!("send close error: {e}");
                            }

                            println!("server sent Close frame to {addr}");
                            break;
                        }
                    }
                    Ok(Message::Ping(payload)) => {
                        println!("ping from client {addr}: {:?}", payload);
                        // Ping を受け取ったら Pong で返す
                        if let Err(e) = write.send(Message::Pong(payload)).await {
                            eprintln!("send pong error: {e}");
                            break;
                        }
                    }
                    Ok(Message::Pong(payload)) => {
                        println!("pong from client {addr}: {:?}", payload);
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
