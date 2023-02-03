// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use iced::{Command, Element, Subscription};
use keechain_core::bitcoin::Network;
use keechain_core::keychain::KeeChain;

mod context;
mod message;
pub mod screen;

pub use self::context::{Context, Stage};
pub use self::message::Message;

pub trait State {
    fn title(&self) -> String;
    fn update(&mut self, ctx: &mut Context, message: Message) -> Command<Message>;
    fn view(&self, ctx: &Context) -> Element<Message>;
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
    fn load(&mut self, _ctx: &Context) -> Command<Message> {
        Command::none()
    }
}

pub fn new_state(context: &Context) -> Box<dyn State> {
    match &context.stage {
        Stage::Home => todo!(),
    }
}

pub struct App {
    state: Box<dyn State>,
    context: Context,
}

impl App {
    pub fn new(network: Network, keechain: KeeChain) -> (Self, Command<Message>) {
        let stage = Stage::default();
        let context = Context::new(stage, network, keechain);
        let app = Self {
            state: new_state(&context),
            context,
        };
        (
            app,
            Command::perform(async {}, move |_| Message::View(stage)),
        )
    }

    pub fn title(&self) -> String {
        self.state.title()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::View(stage) => {
                self.context.set_stage(stage);
                self.state = new_state(&self.context);
                self.state.load(&self.context)
            }
            _ => self.state.update(&mut self.context, message),
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.state.view(&self.context)
    }
}
