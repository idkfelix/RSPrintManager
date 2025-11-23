use printers::common::base::printer::PrinterState;
use serde::ser::SerializeStruct;
use crate::ErrorData;

pub struct Printer(printers::common::base::printer::Printer);

impl serde::Serialize for Printer {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer 
  {
    let printer_state = match &self.0.state {
      PrinterState::OFFLINE => "OFFLINE",
      PrinterState::PAUSED => "PAUSED",
      PrinterState::PRINTING => "PRINTING",
      PrinterState::READY => "READY",
      PrinterState::UNKNOWN => "UNKNOWN",
    };

    let mut state = serializer.serialize_struct("Printer", 5)?;
    state.serialize_field("name", &self.0.name)?;
    state.serialize_field("dataType", &self.0.data_type)?;
    state.serialize_field("description", &self.0.description)?;
    state.serialize_field("isDefault", &self.0.is_default)?;
    state.serialize_field("state", printer_state)?;
    state.end()
  }
}

pub fn get_printers() -> Vec<String> {
  printers::get_printers()
    .as_slice().iter()
    .map(|x| x.name.clone())
    .collect()
}

pub fn get_default_printer() -> Result<Printer, ErrorData> {
  match printers::get_default_printer() {
    Some(printer) => Ok(Printer(printer)),
    None => Err(ErrorData{error: "Default printer not found".into()}),
  }
}

pub fn get_printer(name: String) -> Result<Printer, ErrorData> {
  match printers::get_printer_by_name(name.as_str()) {
    Some(printer) => Ok(Printer(printer)),
    None => Err(ErrorData{error: "Printer not found".into()}),
  }
}