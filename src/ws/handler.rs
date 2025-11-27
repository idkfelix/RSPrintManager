use crate::api::{Request, Response};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
  WebSocketStream, accept_async,
  tungstenite::{Utf8Bytes, protocol::Message},
};

pub async fn handle_connection(stream: TcpStream) {
  let mut ws = accept_async(stream)
    .await
    .expect("Error during WS handshake");

  while let Some(msg) = ws.next().await {
    if let Ok(msg) = msg {
      match msg {
        Message::Text(utf8) => handle_message(&mut ws, utf8).await,
        _ => {},
      }
    }
  }
}

async fn handle_message(ws: &mut WebSocketStream<TcpStream>, msg: Utf8Bytes) {
  if let Ok(req) = serde_json::from_str::<Request>(&msg) {
    let data = req.execute().await;
    let result = serde_json::to_string(&data).unwrap();
    let message = Message::Text(result.into());
    ws.send(message).await.unwrap();
  } else {
    let err = Response::error("Invalid request".into());
    let result = serde_json::to_string(&err).unwrap();
    let message = Message::Text(result.into());
    ws.send(message).await.unwrap();
  }
}
