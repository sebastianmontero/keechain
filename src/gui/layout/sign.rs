// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::path::PathBuf;

use eframe::egui::{Align, Layout, RichText, Ui};
use rfd::FileDialog;

use crate::core::command;
use crate::core::types::Psbt;
use crate::gui::component::{Button, Heading, Version};
use crate::gui::theme::color::{DARK_GREEN, DARK_RED, ORANGE, RED};
use crate::gui::{AppData, AppStage, Menu};

#[allow(dead_code)]
#[derive(Clone)]
pub struct PsbtFile {
    psbt: Psbt,
    path: PathBuf,
}

#[derive(Clone, Default)]
pub struct SignLayoutData {
    psbt_file: Option<PsbtFile>,
    error: Option<String>,
    finish: bool,
}

impl SignLayoutData {
    pub fn clear(&mut self) {
        self.psbt_file = None;
        self.error = None;
        self.finish = false;
    }
}

pub fn update_layout(app: &mut AppData, ui: &mut Ui) {
    if app.seed.is_none() {
        app.set_stage(AppStage::Start);
    }

    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        ui.set_max_width(ui.available_width() - 20.0);

        Heading::new("Sign").render(ui);

        ui.add_space(15.0);

        if let Some(error) = &app.layouts.sign.error {
            ui.label(RichText::new(error).color(RED));
            ui.add_space(7.0);
        }

        let is_signed: bool = app.layouts.sign.finish;
        let is_ready_to_sign: bool = app.layouts.sign.psbt_file.is_some();

        if !is_ready_to_sign && !is_signed {
            let button = Button::new("Select PSBT file")
                .background_color(DARK_GREEN)
                .render(ui);

            if button.clicked() {
                if let Some(path) = FileDialog::new().add_filter("psbt", &["psbt"]).pick_file() {
                    match command::psbt::decode_file(path.clone(), app.network) {
                        Ok(psbt) => {
                            app.layouts.sign.error = None;
                            app.layouts.sign.psbt_file = Some(PsbtFile { psbt, path });
                        }
                        Err(e) => app.layouts.sign.error = Some(e.to_string()),
                    }
                }
            }
        }

        if is_ready_to_sign && !is_signed {
            if let Some(psbt_file) = app.layouts.sign.psbt_file.as_ref() {
                if Button::new("Sign")
                    .background_color(ORANGE)
                    .render(ui)
                    .clicked()
                {
                    if let Some(seed) = app.seed.as_ref() {
                        match command::psbt::sign_file_from_seed(
                            seed,
                            app.network,
                            psbt_file.path.clone(),
                        ) {
                            Ok(finalized) => {
                                if finalized {
                                    app.layouts.sign.clear();
                                    app.layouts.sign.finish = true;
                                } else {
                                    app.layouts.sign.error =
                                        Some("PSBT signing not finalized".to_string());
                                }
                            }
                            Err(e) => app.layouts.sign.error = Some(e.to_string()),
                        }
                    } else {
                        app.layouts.sign.error = Some("Seed not loaded".to_string());
                    }
                }
            }

            ui.add_space(5.0);

            if Button::new("Clear")
                .background_color(DARK_RED)
                .render(ui)
                .clicked()
            {
                app.layouts.sign.clear();
            }
        }

        if is_signed {
            ui.label(RichText::new("PSBT signed!").color(DARK_GREEN));
            ui.add_space(5.0);
            if Button::new("Sign again").render(ui).clicked() {
                app.layouts.sign.clear();
            }
        }

        ui.add_space(5.0);

        if Button::new("Back").render(ui).clicked() {
            app.layouts.sign.clear();
            app.stage = AppStage::Menu(Menu::Main);
        }
    });

    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        Version::new().render(ui)
    });
}
