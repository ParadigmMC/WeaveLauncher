use anyhow::Result;

use self::app::WeaveLauncher;

pub mod widgets;
pub mod pages;
pub mod app;

pub fn run_eframe() -> Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Weave Launcher",
        native_options,
        Box::new(|cc| Box::new(WeaveLauncher::new(cc))),
    ).expect("EFrame Err");

    Ok(())
}