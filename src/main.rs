mod ws;
mod tray;
mod command;
mod printing;

use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use crate::{command::{Command, CommandType}, ws::{WsResult, SendJson}};
use tokio_tungstenite::{
  tungstenite::{Utf8Bytes, protocol::Message},
  WebSocketStream, 
  accept_async, 
};

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
        ws.send_json(printing::get_printers()).await;
      },
      CommandType::GetDefaultPrinter => {
        ws.send_json(printing::get_default_printer()).await;
      },
      CommandType::GetPrinter => {
        if let Some(name) = command.value {
          ws.send_json(printing::get_printer(name)).await;
        }
      },
    }
  } else {
    ws.send_json(WsResult::Err::<String, String>(
      String::from("Invalid Command")
    )).await;
  }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let port = 3000;
  let addr = format!("127.0.0.1:{}", port);
  let _tray_icon = tray::init_tray().unwrap();

  let listener = TcpListener::bind(addr).await?;
  while let Ok((stream, _)) = listener.accept().await {
    tokio::spawn(handle_connection(stream));
  }
  Ok(())
}