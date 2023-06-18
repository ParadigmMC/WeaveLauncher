use anyhow::Result;
use egui::Color32;
use egui_modal::{Modal, Icon};
use poll_promise::Promise;

use crate::api::WEAVE;

use super::pages::create_instance::CreateInstancePage;

pub enum Page {
    Home,
    CreateInstance,
}

pub struct WeaveLauncher {
    pub page: Page,
}

impl Default for WeaveLauncher {
    fn default() -> Self {
        Self {
            page: Page::CreateInstance,
        }
    }
}

impl WeaveLauncher {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for WeaveLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut err_modal = Modal::new(ctx, "err_modal");

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                if ui.button("Create new instance").clicked() {
                    dbg!("Creating new instance...");
                }

                if ui.button("Open Instances Folder").clicked() {
                    dbg!("Opening instances folder");
                    if let Err(e) = opener::open(&WEAVE.blocking_read().instances.path) {
                        err_modal.open_dialog(Some("Error"), Some(e.to_string()), Some(Icon::Error));
                    }
                }

                if ui.button("Settings").clicked() {
                    dbg!("Opening settings");
                }

                if ui.button("Refresh Instances").clicked() {
                    if let Err(e) =  WEAVE.blocking_write().instances.reload() {
                        err_modal.open_dialog(Some("Error"), Some(e.to_string()), Some(Icon::Error));
                    }
                }

                if ui.button("New Instance").clicked() {

                }
            });
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            match self.page {
                Page::Home => {
                    ui.heading("Welcome");
                },
                Page::CreateInstance => {
                    ui.add(&mut CreateInstancePage::new());
                },
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("instances_grid").show(ui, |ui| {
                for instance in WEAVE.blocking_read().instances.instances.iter() {
                    ui.vertical_centered(|ui| {
                        ui.label("Instance:");
                        ui.label(&instance.name);
                    });
                }
            });
        });

        err_modal.show_dialog();
    }
}
