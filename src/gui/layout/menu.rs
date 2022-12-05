// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use eframe::egui::{Align, CentralPanel, Context, Layout};
use eframe::Frame;

use crate::gui::component::{Button, Heading, Version};
use crate::gui::{AppData, AppStage, Menu};

pub fn update_layout(app: &mut AppData, menu: Menu, ctx: &Context, frame: &mut Frame) {
    if app.seed.is_none() {
        app.set_stage(AppStage::Start);
    }

    CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            Heading::new("Menu").render(ui);

            ui.add_space(15.0);

            match menu {
                Menu::Main => {
                    if Button::new("Advanced").render(ui).clicked() {
                        app.stage = AppStage::Menu(Menu::Advanced);
                    }
                    ui.add_space(5.0);
                    if Button::new("Setting").render(ui).clicked() {
                        app.stage = AppStage::Menu(Menu::Setting);
                    }
                    ui.add_space(5.0);
                    if Button::new("Lock").render(ui).clicked() {
                        app.stage = AppStage::Start;
                    }
                    ui.add_space(5.0);
                    if Button::new("Exit").render(ui).clicked() {
                        frame.close();
                    }
                }
                Menu::Advanced => {
                    if Button::new("Danger").render(ui).clicked() {
                        app.stage = AppStage::Menu(Menu::Danger);
                    }
                    ui.add_space(5.0);
                    if Button::new("Back").render(ui).clicked() {
                        app.stage = AppStage::Menu(Menu::Main);
                    }
                }
                Menu::Setting => {
                    if Button::new("Back").render(ui).clicked() {
                        app.stage = AppStage::Menu(Menu::Main);
                    }
                }
                Menu::Danger => {
                    if Button::new("Back").render(ui).clicked() {
                        app.stage = AppStage::Menu(Menu::Advanced);
                    }
                }
            }
        });

        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            Version::new().render(ui)
        });
    });
}