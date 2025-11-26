use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use super::result::WsResult;
use crate::command::Command;
use tokio_tungstenite::{
  tungstenite::{Utf8Bytes, protocol::Message},
  WebSocketStream, 
  accept_async, 
};

pub async fn handle_connection(stream: TcpStream) {
  let mut ws = accept_async(stream).await
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
  if let Ok(command) = serde_json::from_str::<Command>(&msg) {
    match command.execute().await {
      Ok(result) => {
        let message = Message::Text(result.into());
        ws.send(message).await.unwrap();
      },
      Err(err) => {
        let err = WsResult::error(&err);
        let result = serde_json::to_string(&err).unwrap();
        let message = Message::Text(result.into());
        ws.send(message).await.unwrap();
      },
    }
  } else {
    let err = WsResult::error("Invalid Command");
    let result = serde_json::to_string(&err).unwrap();
    let message = Message::Text(result.into());
    ws.send(message).await.unwrap();
  }
}