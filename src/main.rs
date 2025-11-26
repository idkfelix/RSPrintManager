use rspm::{ui::tray::init_tray, ws::handler::handle_connection};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let port = 3000;
  let addr = format!("127.0.0.1:{}", port);
  let _tray_icon = init_tray().unwrap();

  let listener = TcpListener::bind(addr).await?;
  while let Ok((stream, _)) = listener.accept().await {
    tokio::spawn(handle_connection(stream));
  }
  Ok(())
}