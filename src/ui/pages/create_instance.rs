use anyhow::Result;
use egui::Widget;
use mcapi::vanilla::VersionManifest;
use poll_promise::Promise;

use crate::api::WEAVE;

pub struct CreateInstancePage {
    pub name: String,

    pub selected_version: String,
    pub versions: Option<Promise<Result<VersionManifest>>>,

    pub err_text: Option<String>,
}

impl CreateInstancePage {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for CreateInstancePage {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            versions: None,
            selected_version: "".to_owned(),
            err_text: None,
        }
    }
}

impl Widget for &mut CreateInstancePage {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let promise = self.versions.get_or_insert_with(|| {
            Promise::spawn_async(async move {
                let weave = WEAVE.read().await;
                Ok(mcapi::vanilla::fetch_version_manifest(&weave.http_client).await?)
            })
        });

        ui.vertical(|ui| {
            ui.heading("Create Instance");

            ui.text_edit_singleline(&mut self.name);

            egui::ComboBox::from_label("Version")
                .selected_text(format!("{:?}", match promise.ready() {
                    Some(Ok(_)) => self.selected_version.clone(),
                    Some(Err(_)) => "Error".to_owned(),
                    None => "Loading...".to_owned(),
                }))
                .show_ui(ui, |ui| {
                    match promise.ready() {
                        Some(Ok(manifest)) => {
                            for ver in manifest.versions.iter() {
                                ui.selectable_value(
                                    &mut self.selected_version,
                                    ver.id.clone(),
                                    {
                                        if ver.id == manifest.latest.release {
                                            ver.id.to_owned() + "(latest release)"
                                        } else if ver.id == manifest.latest.snapshot {
                                            ver.id.to_owned() + " (latest snapshot)"
                                        } else {
                                            ver.id.to_owned()
                                        }
                                    }
                                );
                            }
                        },

                        Some(Err(e)) => {
                            ui.label(e.to_string());
                        },

                        None => {
                            ui.spinner();
                        },
                    }
                });

            if ui.button("Create").clicked() {
                eprintln!("Creating instance '{}' with version {}", self.name, self.selected_version);

                let weave = WEAVE.blocking_read();
                match weave.instances.create(&self.name) {
                    Ok(mut inst) => {
                        inst.mc_version = self.selected_version.clone();
                        if let Err(e) = inst.save() {
                            self.err_text = Some(e.to_string());
                        }
                    },

                    Err(e) => self.err_text = Some(e.to_string()),
                }
            };

            if let Some(e) = &self.err_text {
                ui.label(e);
            }
        }).response
    }
}
