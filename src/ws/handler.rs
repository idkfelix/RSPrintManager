use futures_util::StreamExt;
use tokio::net::TcpStream;
use super::{result::WsResult, send_json::SendJson};
use crate::command::{Command, CommandType};
use crate::printing;
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
    match command.r#type {
      CommandType::GetPrinters => {
        ws.send_json(printing::get_printers()).await;
      },
      CommandType::GetDefaultPrinter => {
        ws.send_json(printing::get_default_printer()).await;
      },
      CommandType::GetPrinter => {
        if let Some(name) = command.value {
          ws.send_json(printing::get_printer(name)).await;
        } else {
          ws.send_json(WsResult::Err::<String, String>(
            String::from("value field missing")
          )).await;         
        }
      },
      CommandType::PrintRaw => {
        if let None = command.printer { 
          ws.send_json(WsResult::Err::<String, String>(
            String::from("printer field missing")
          )).await;
        } else if let None = command.value { 
          ws.send_json(WsResult::Err::<String, String>(
            String::from("value field missing")
          )).await;
        } else {
          ws.send_json(printing::print_raw(
            command.printer.unwrap(), 
            command.value.unwrap().as_bytes(),
          )).await;
        }
      }
    }
  } else {
    ws.send_json(WsResult::Err::<String, String>(
      String::from("Invalid Command")
    )).await;
  }
}