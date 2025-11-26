#[derive(Debug)]
pub enum WsResult<T, E> where
  T: serde::Serialize + std::fmt::Debug,
  E: serde::Serialize + std::fmt::Debug, 
{
  Ok(T),
  Err(E),
}

use WsResult::*;
use serde::ser::SerializeMap;

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