use printers::common::base::job::PrinterJobOptions;
use super::Printer;
use crate::ws::result::WsResult;
use WsResult::*;

pub fn get_printers() -> WsResult<Vec<String>, ()> {
  let list: Vec<String> = printers::get_printers()
    .into_iter()
    .map(|x| x.name.clone())
    .collect();
  Ok(list)
}

pub fn get_default_printer() -> WsResult<Printer, String> {
  match printers::get_default_printer() {
    Some(printer) => Ok(Printer(printer)),
    None => Err("Default printer not found".into()),
  }
}

pub fn get_printer(name: String) -> WsResult<Printer, String> {
  match printers::get_printer_by_name(name.as_str()) {
    Some(printer) => Ok(Printer(printer)),
    None => Err("Printer not found".into()),
  }
}

pub fn print_raw(printer: String, data: &[u8]) -> WsResult<u64, String> {
  match printers::get_printer_by_name(printer.as_str()) {
    Some(printer) => {
      match printer.print(data, PrinterJobOptions {
        name: None,
        raw_properties: &[]
      }) {
        Result::Ok(value) => Ok(value),
        Result::Err(error) => Err(error.to_string())
      }
    },
    None => Err(String::from("Printer not found"))
  }
}