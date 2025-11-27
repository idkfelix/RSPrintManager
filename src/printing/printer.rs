use printers::common::base::printer::{self, PrinterState};

#[derive(Debug, serde::Serialize, ts_rs::TS)]
pub struct Printer {
  name: String,
  data_type: String,
  description: String,
  is_default: bool,
  state: String,
}

impl From<printer::Printer> for Printer {
  fn from(printer: printer::Printer) -> Self {
    let state = match &printer.state {
      PrinterState::OFFLINE => "OFFLINE",
      PrinterState::PAUSED => "PAUSED",
      PrinterState::PRINTING => "PRINTING",
      PrinterState::READY => "READY",
      PrinterState::UNKNOWN => "UNKNOWN",
    }
    .to_string();

    Printer {
      name: printer.name,
      data_type: printer.data_type,
      description: printer.description,
      is_default: printer.is_default,
      state,
    }
  }
}
