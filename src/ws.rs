#[derive(Debug)]
pub enum WsResult<T, E> where
  T: serde::Serialize + std::fmt::Debug,
  E: serde::Serialize + std::fmt::Debug, 
{
  Ok(T),
  Err(E),
}

use WsResult::*;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use serde::ser::SerializeMap;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

impl<T,E> serde::Serialize for WsResult<T,E> where 
  T: serde::Serialize + std::fmt::Debug,
  E: serde::Serialize + std::fmt::Debug, 
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
    S: serde::Serializer 
  {
    let mut map = serializer.serialize_map(Some(1))?;
    match self {
      Ok(data) => map.serialize_entry("data", data)?,
      Err(error) => map.serialize_entry("error", error)?,
    }
    map.end()
  }
}

pub trait SendJson {
  async fn send_json<T,E>(&mut self, res: WsResult<T,E>) where 
    T: serde::Serialize + std::fmt::Debug,
    E: serde::Serialize + std::fmt::Debug
  ;
}

impl SendJson for WebSocketStream<TcpStream> {
  async fn send_json<T,E>(&mut self, res: WsResult<T,E>) where 
    T: serde::Serialize + std::fmt::Debug,
    E: serde::Serialize + std::fmt::Debug
  {
    let json = serde_json::to_string(&res).unwrap();
    let json_message = Message::Text(json.into());
    self.send(json_message).await.unwrap();
  }
}