#[derive(Debug, serde::Deserialize)]
pub struct Command {
  pub cmd: CommandType,
  pub value: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
  GetPrinters,
  GetDefaultPrinter,
  GetPrinter,
}

impl<'de> serde::Deserialize<'de> for CommandType {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de> 
  {
    match String::deserialize(deserializer)?.as_str() {
      "get_printers" => Ok(Self::GetPrinters),
      "get_default_printer" => Ok(Self::GetDefaultPrinter),
      "get_printer" => Ok(Self::GetPrinter),
      _ => Err(serde::de::Error::custom("Invalid Command")),
    }
  }
}