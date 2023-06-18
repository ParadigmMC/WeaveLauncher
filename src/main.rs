// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use tokio::runtime::Runtime;

pub mod api;
pub mod util;
pub mod ui;

fn main() -> Result<()> {
    env_logger::init();

    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    ui::run_eframe()?;

    Ok(())
}
