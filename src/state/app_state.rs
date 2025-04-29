use crate::message::main_message::MainMessage;
use crate::widget::main_screen::MainScreen;
use iced::widget::{button, column, container, row};
use iced::{Element, Fill, Theme};

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
        format!(
            "{}",
            match self.screen {
                MainScreen::Home => "Home",
                MainScreen::Register => "Register",
                MainScreen::MonthlySummary => "Monthly Summary",
            }
        )
    }

    pub(crate) fn update(&mut self, message: MainMessage) {
        match message {
            MainMessage::GoToHome => self.screen = MainScreen::Home,
            MainMessage::GoToRegister => self.screen = MainScreen::Register,
            MainMessage::GoToMonthlySummary => self.screen = MainScreen::MonthlySummary,
        }
    }
    pub(crate) fn view(&self) -> Element<MainMessage> {
        match self.screen {
            MainScreen::Home => {
                // Create the home screen view
                let register_button = button("Register").on_press(MainMessage::GoToRegister);
                let monthly_summary_button =
                    button("Monthly Summary").on_press(MainMessage::GoToMonthlySummary);
                let home_row = row![register_button, monthly_summary_button].spacing(20);

                container(home_row)
                    .center_x(Fill)
                    .center_y(Fill)
                    .padding(20)
                    .into()
            }
            _ => {
                // Create the other screen views
                let back_button = button("Back").on_press(MainMessage::GoToHome);
                let content = column![back_button];
                Element::new(content)
            }
        }
    }

    pub(crate) fn theme(&self) -> Theme {
        match self.screen {
            MainScreen::Home => Theme::default(),
            MainScreen::Register => Theme::Dark,
            MainScreen::MonthlySummary => Theme::Dark,
        }
    }
}
