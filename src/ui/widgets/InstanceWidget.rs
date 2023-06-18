use egui::Widget;

use crate::api::instances::Instance;

pub struct InstanceWidget {
    instance: Instance,
}

impl Widget for &mut InstanceWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered(|ui| {
            ui.label("Instance:");
            ui.label(&self.instance.name);
        }).response
    }
}