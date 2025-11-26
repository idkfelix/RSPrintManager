use super::result::WsResult;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub trait SendJson {
  fn send_json<T, E>(
    &mut self,
    res: WsResult<T, E>,
  ) -> impl std::future::Future<Output = ()> + Send
  where
    T: serde::Serialize + std::fmt::Debug,
    E: serde::Serialize + std::fmt::Debug;
}

impl SendJson for WebSocketStream<TcpStream> {
  fn send_json<T, E>(
    &mut self,
    res: WsResult<T, E>,
  ) -> impl std::future::Future<Output = ()> + Send
  where
    T: serde::Serialize + std::fmt::Debug,
    E: serde::Serialize + std::fmt::Debug,
  {
    let json = serde_json::to_string(&res).unwrap();
    let json_message = Message::Text(json.into());
    async move {
      self.send(json_message).await.unwrap();
    }
  }
}