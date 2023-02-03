// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{executor, Application, Command, Element, Settings, Theme};

mod app;
mod component;
mod start;
mod theme;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.min_size = Some((600, 600));
    settings.default_font = Some(theme::font::REGULAR_BYTES);
    KeeChainApp::run(settings)
}

pub struct KeeChainApp {
    state: State,
}
pub enum State {
    Start(start::Start),
    App(app::App),
}

#[derive(Debug, Clone)]
pub enum Message {
    Start(Box<start::Message>),
    App(Box<app::Message>),
}

impl Application for KeeChainApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let stage = start::Start::new();
        (
            Self {
                state: State::Start(stage.0),
            },
            stage.1.map(|m| m.into()),
        )
    }

    fn title(&self) -> String {
        match &self.state {
            State::Start(auth) => auth.title(),
            State::App(app) => app.title(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match (&mut self.state, message) {
            (State::Start(start), Message::Start(msg)) => {
                let (command, stage_to_move) = start.update(*msg);
                if let Some(stage) = stage_to_move {
                    *self = stage;
                }
                command.map(|m| m.into())
            }
            (State::App(app), Message::App(msg)) => app.update(*msg).map(|m| m.into()),
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            State::Start(start) => start.view().map(|m| m.into()),
            State::App(app) => app.view().map(|m| m.into()),
        }
    }
}
