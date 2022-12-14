// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use eframe::egui::{Align, Key, Layout, RichText, Ui};

use crate::component::{Button, Heading, InputField, Version};
use crate::theme::color::{ORANGE, RED};
use crate::{AppState, Menu, Stage};

#[derive(Default)]
pub struct ChangePasswordState {
    new_password: String,
    confirm_new_password: String,
    error: Option<String>,
}

impl ChangePasswordState {
    pub fn clear(&mut self) {
        self.new_password = String::new();
        self.confirm_new_password = String::new();
        self.error = None;
    }
}

pub fn update_layout(app: &mut AppState, ui: &mut Ui) {
    if app.keechain.is_none() {
        app.set_stage(Stage::Start);
    }

    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        ui.set_max_width(ui.available_width() - 20.0);

        Heading::new("Change password").render(ui);

        ui.add_space(15.0);

        InputField::new("New password")
            .placeholder("New password")
            .is_password()
            .render(ui, &mut app.layouts.change_password.new_password);

        ui.add_space(7.0);

        InputField::new("Confirm new password")
            .placeholder("Confirm new password")
            .is_password()
            .render(ui, &mut app.layouts.change_password.confirm_new_password);

        ui.add_space(7.0);

        if let Some(error) = &app.layouts.change_password.error {
            ui.label(RichText::new(error).color(RED));
        }

        ui.add_space(15.0);

        let is_ready: bool = !app.layouts.change_password.new_password.is_empty()
            && !app.layouts.change_password.confirm_new_password.is_empty();

        let button = Button::new("Rename")
            .background_color(ORANGE)
            .enabled(is_ready)
            .render(ui);

        if is_ready && (ui.input().key_pressed(Key::Enter) || button.clicked()) {
            if app.layouts.change_password.new_password
                != app.layouts.change_password.confirm_new_password
            {
                app.layouts.change_password.error = Some("Passwords not match".to_string());
            } else {
                match app.keechain.as_mut() {
                    Some(keechain) => {
                        match keechain.change_password(|| {
                            Ok(app.layouts.change_password.new_password.clone())
                        }) {
                            Ok(_) => {
                                app.layouts.change_password.clear();
                                app.stage = Stage::Menu(Menu::Setting);
                            }
                            Err(e) => app.layouts.change_password.error = Some(e.to_string()),
                        }
                    }
                    None => {
                        app.layouts.change_password.error =
                            Some("Impossible to get keechain".to_string())
                    }
                }
            }
        }

        ui.add_space(5.0);
        if Button::new("Back").render(ui).clicked() {
            app.layouts.rename_keychain.clear();
            app.stage = Stage::Menu(Menu::Setting);
        }
    });

    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        Version::new().render(ui)
    });
}
