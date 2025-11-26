use printers::common::base::printer::PrinterState;
use serde::ser::SerializeStruct;

#[derive(Debug)]
pub struct Printer(
  pub printers::common::base::printer::Printer
);

impl std::ops::Deref for Printer {
  type Target = printers::common::base::printer::Printer;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

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
    state.serialize_field("name", &self.name)?;
    state.serialize_field("dataType", &self.data_type)?;
    state.serialize_field("description", &self.description)?;
    state.serialize_field("isDefault", &self.is_default)?;
    state.serialize_field("state", printer_state)?;
    state.end()
  }
}