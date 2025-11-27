use crate::printing::Printer;

#[derive(Debug, serde::Serialize, ts_rs::TS)]
#[serde(untagged)]
#[ts(export)]
pub enum Response {
  Error {
    error: String,
  },
  Printers {
    printers: Vec<String>,
  },
  PrintJob {
    id: u64,
  },
  Printer {
    #[ts(inline)]
    printer: Printer,
  },
}

impl Response {
  pub fn error(error: &str) -> Self {
    Response::Error {
      error: error.to_string(),
    }
  }

  pub fn missing_field(field: &str) -> Self {
    Response::Error {
      error: format!("Missing field '{}'", field),
    }
  }
}
