use printers::common::base::job::PrinterJobOptions;
use crate::ws::result::WsResult::{self, Ok, Err};

use super::Printer;

pub fn get_printers() -> WsResult<Vec<String>, String>{
  let list = printers::get_printers()
    .into_iter()
    .map(|x| x.name.clone())
    .collect();
  Ok(list)
}

pub fn get_default_printer() -> WsResult<Printer, String> {
  match printers::get_default_printer() {
    Some(printer) => Ok(Printer(printer)),
    None => Err("Default printer not found".to_string()),
  }
}

pub fn get_printer(name: String) -> WsResult<Printer, String> {
  match printers::get_printer_by_name(name.as_str()) {
    Some(printer) => Ok(Printer(printer)),
    None => Err(format!("Printer '{}' not found", name)),
  }
}

pub fn print_raw(printer: String, data: &[u8]) -> WsResult<u64, String> {
  let opts = PrinterJobOptions{ name: None, raw_properties: &[] };
  match get_printer(printer) {
    Ok(printer) => {
      match printer.print(data, opts) {
        Result::Ok(job_id) => Ok(job_id),
        Result::Err(error) => Err(error.to_string()),
      }
    },
    Err(error) => Err(error),
  }
}