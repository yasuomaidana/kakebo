use crate::message::main_message::MainMessage;
use crate::widget::main_screen::MainScreen;
use iced::{Element, Theme};

#[derive(Default)]
pub struct AppState {
    pub screen: MainScreen,
}

impl Default for MainScreen {
    fn default() -> Self {
        MainScreen::Home
    }
}

impl AppState {
    pub(crate) fn title(&self) -> String {
        self.screen.title()
    }

    pub(crate) fn update(&mut self, message: MainMessage) {
        match message {
            MainMessage::GoToHome => self.screen = MainScreen::Home,
            MainMessage::GoToRegister => {
                self.screen = MainScreen::RegisterWidget(Default::default())
            }
            MainMessage::GoToMonthlySummary => self.screen = MainScreen::MonthlySummary,
            MainMessage::RegisterMessage(register_message) => match self.screen {
                MainScreen::RegisterWidget(ref mut prev_state) => {
                    prev_state.update(register_message);
                }
                _ => {
                    self.screen = MainScreen::RegisterWidget(Default::default());
                }
            },
        }
    }
    pub(crate) fn view(&self) -> Element<MainMessage> {
        self.screen.view()
    }

    pub(crate) fn theme(&self) -> Theme {
        match self.screen {
            MainScreen::Home => Theme::default(),
            MainScreen::RegisterWidget(_) => Theme::Dark,
            MainScreen::MonthlySummary => Theme::Dark,
        }
    }
}
