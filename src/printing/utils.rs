use super::Printer;
use crate::api::Response;
use printers::common::base::job::PrinterJobOptions;

pub fn get_printers() -> Response {
  let list = printers::get_printers()
    .into_iter()
    .map(|x| x.name.clone())
    .collect();
  Response::Printers { printers: list }
}

pub fn get_default_printer() -> Response {
  match printers::get_default_printer() {
    Some(p) => Response::Printer {
      printer: Printer::from(p),
    },
    None => Response::error("Default printer not found"),
  }
}

pub fn get_printer(name: String) -> Response {
  match printers::get_printer_by_name(&name) {
    Some(p) => Response::Printer {
      printer: Printer::from(p),
    },
    None => Response::error(&format!("Printer '{}' not found", name)),
  }
}

pub fn print_raw(name: String, data: &[u8]) -> Response {
  let opts = PrinterJobOptions {
    name: None,
    raw_properties: &[],
  };
  match printers::get_printer_by_name(&name) {
    Some(printer) => match printer.print(data, opts) {
      Result::Ok(id) => Response::PrintJob { id },
      Result::Err(error) => Response::error(error),
    },
    None => Response::error(&format!("Printer '{}' not found", name)),
  }
}
