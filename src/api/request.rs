use super::Response;
use crate::printing;

#[derive(Debug, serde::Deserialize, ts_rs::TS)]
#[serde(tag = "type")]
#[ts(export)]
pub enum Request {
  GetPrinters,
  GetDefaultPrinter,
  GetPrinter {
    #[ts(type = "string")]
    name: Option<String>,
  },
  PrintRaw {
    #[ts(type = "string")]
    name: Option<String>,
    #[ts(type = "string")]
    data: Option<String>,
  },
}

impl Request {
  pub async fn execute(&self) -> Response {
    match &self {
      Request::GetPrinters => printing::get_printers(),
      Request::GetDefaultPrinter => printing::get_default_printer(),
      Request::GetPrinter { name } => match name {
        Some(name) => printing::get_printer(name.to_string()),
        None => Response::missing_field("name"),
      },
      Request::PrintRaw { name, data } => match (name, data) {
        (Some(name), Some(data)) => printing::print_raw(name.to_string(), data.as_bytes()),
        (None, _) => Response::missing_field("name"),
        (_, None) => Response::missing_field("data"),
      },
    }
  }
}
