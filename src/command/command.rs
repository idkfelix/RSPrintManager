use crate::{printing, ws::result::WsResult};
use super::types::CommandType;

#[derive(Debug, serde::Deserialize)]
pub struct Command {
  #[serde(rename = "type")]
  pub r#type: CommandType,
  pub value: Option<String>,
  pub printer: Option<String>,
}

impl Command {
  pub async fn execute(&self) -> Result<String, String> {
    match self.r#type {
      CommandType::GetPrinters => {
        let result = printing::get_printers();
        serde_json::to_string(&result).map_err(|e| e.to_string())
      },
      CommandType::GetDefaultPrinter => {
        let result = printing::get_default_printer();
        serde_json::to_string(&result).map_err(|e| e.to_string())
      },
      CommandType::GetPrinter => {
        if let Some(name) = &self.value {
          let result = printing::get_printer(name.to_string());
          serde_json::to_string(&result).map_err(|e| e.to_string())
        } else {
          let result = WsResult::error("value field missing");
          serde_json::to_string(&result).map_err(|e| e.to_string())
        }
      },
      CommandType::PrintRaw => {
        if let (Some(printer), Some(value)) = (&self.printer, &self.value) {
          let result = printing::print_raw(printer.to_string(), value.as_bytes());
          serde_json::to_string(&result).map_err(|e| e.to_string())
        } else {
          let result = WsResult::error("missing printer or value field");
          serde_json::to_string(&result).map_err(|e| e.to_string())
        }
      },
    }
  }
}