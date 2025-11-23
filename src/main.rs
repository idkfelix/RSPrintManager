mod tray;
mod command;
mod printing;

use std::io::Error;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use crate::command::{Command, CommandType};
use tokio_tungstenite::{
  tungstenite::{Utf8Bytes, protocol::Message},
  WebSocketStream, 
  accept_async, 
};

#[derive(Debug, serde::Serialize)]
pub struct ErrorData {
  error: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let port = 3000;
  let addr = format!("127.0.0.1:{}", port);
  let _tray_icon = tray::init_tray().unwrap();

  let listener = TcpListener::bind(addr).await?;
  while let Ok((stream, _)) = listener.accept().await {
    tokio::spawn(handle_connection(stream));
  }
  Ok(())
}

async fn handle_connection(stream: TcpStream) {
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
    match command.cmd {
      CommandType::GetPrinters => {
        let data = printing::get_printers();
        send_json(ws, &data).await;
      },
      CommandType::GetDefaultPrinter => {
        match printing::get_default_printer() {
          Ok(data) => send_json(ws, &data).await,
          Err(error) => send_json(ws, &error).await,
        }
      },
      CommandType::GetPrinter => {
        if let Some(name) = command.value {
          match printing::get_printer(name) {
            Ok(data) => send_json(ws, &data).await,
            Err(error) => send_json(ws, &error).await,
          }
        }
      },
    }
  } else {
    let err = ErrorData{error: "Invalid Command".into()};
    send_json(ws, &err).await;
  }
}

async fn send_json(ws: &mut WebSocketStream<TcpStream>, data: &impl serde::Serialize) {
  match serde_json::to_string(&data) {
    Ok(json_string) => {
      let json_message = Message::Text(json_string.into());
      ws.send(json_message).await.unwrap()
    },
    Err(_) => {
      let err_message = Message::Text("{\"error\":\"Failed to serialize\"}".into());
      ws.send(err_message).await.unwrap()
    },
  }
}