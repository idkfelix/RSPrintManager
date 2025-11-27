use tray_icon::{Icon, Result, TrayIcon, TrayIconBuilder};

static ICON_DATA: &[u8] = include_bytes!("../../icon.png");

pub fn init_tray() -> Result<TrayIcon> {
  let icon = load_icon();
  TrayIconBuilder::new()
    .with_tooltip("RSPrintManager")
    .with_icon(icon)
    .build()
}

fn load_icon() -> Icon {
  let image = image::load_from_memory(ICON_DATA)
    .expect("Failed to load icon data")
    .into_rgba8();
  let (width, height) = image.dimensions();
  let rgba = image.into_raw();

  Icon::from_rgba(rgba, width, height)
    .expect("Failed to open icon")
}
