use std::path::Path;
use tray_icon::{Icon, Result, TrayIcon, TrayIconBuilder};

pub fn init_tray() -> Result<TrayIcon> {
  let icon = load_icon(Path::new("icon.png"));
  TrayIconBuilder::new()
    .with_tooltip("RSPrintManager")
    .with_icon(icon)
    .build()
}

fn load_icon(path: &Path) -> Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::open(path)
      .expect("Failed to open icon path")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
