pub mod api;
pub mod printing;
pub mod ui;
pub mod ws;

use tokio::net::TcpListener;
use ui::tray::init_tray;
use ws::handler::handle_connection;

pub const DEFAULT_PORT: u16 = 3000;
pub const DEFAULT_ADDRESS: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let addr = format!("{}:{}", DEFAULT_ADDRESS, DEFAULT_PORT);
  let _tray_icon = init_tray().unwrap();

  let listener = TcpListener::bind(addr).await?;
  while let Ok((stream, _)) = listener.accept().await {
    tokio::spawn(handle_connection(stream));
  }
  Ok(())
}
