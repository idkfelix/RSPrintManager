use rspm::{ui::tray::init_tray, ws::handler::handle_connection};
use rspm::config::{DEFAULT_PORT, DEFAULT_ADDRESS};
use tokio::net::TcpListener;

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