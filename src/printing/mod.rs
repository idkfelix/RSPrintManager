mod models;
mod utils;

pub use models::Printer;
pub use utils::{
  get_printers,
  get_printer,
  get_default_printer,
  print_raw,
};