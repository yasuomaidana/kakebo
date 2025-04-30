use crate::message::main_message::Message;
use crate::widget::main_screen::Screen;
use iced::{Element, Theme};

#[derive(Default)]
pub struct State {
    pub screen: Screen,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Home
    }
}

impl State {
    pub(crate) fn title(&self) -> String {
        self.screen.title()
    }

    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::GoToHome => self.screen = Screen::Home,
            Message::GoToRegister => self.screen = Screen::RegisterWidget(Default::default()),
            Message::GoToMonthlySummary => self.screen = Screen::MonthlySummary,
            Message::RegisterMessage(register_message) => match self.screen {
                Screen::RegisterWidget(ref mut prev_state) => {
                    prev_state.update(register_message);
                }
                _ => {
                    self.screen = Screen::RegisterWidget(Default::default());
                }
            },
        }
    }
    pub(crate) fn view(&self) -> Element<Message> {
        self.screen.view()
    }

    pub(crate) fn theme(&self) -> Theme {
        match self.screen {
            Screen::Home => Theme::default(),
            Screen::RegisterWidget(_) => Theme::Dark,
            Screen::MonthlySummary => Theme::Dark,
        }
    }
}
