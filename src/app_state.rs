use crate::message::Message;
use crate::screen::Screen;
use iced::widget::{button, column, container, row};
use iced::{Element, Fill};

#[derive(Default)]
pub struct AppState {
    pub screen: Screen,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Home
    }
}

impl AppState {
    pub(crate) fn title(&self) -> String {
        format!(
            "{}",
            match self.screen {
                Screen::Home => "Home",
                Screen::Register => "Register",
                Screen::MonthlySummary => "Monthly Summary",
            }
        )
    }

    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::GoToHome => self.screen = Screen::Home,
            Message::GoToRegister => self.screen = Screen::Register,
            Message::GoToMonthlySummary => self.screen = Screen::MonthlySummary,
        }
    }
    pub(crate) fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Home => {
                // Create the home screen view
                let register_button = button("Register").on_press(Message::GoToRegister);
                let monthly_summary_button =
                    button("Monthly Summary").on_press(Message::GoToMonthlySummary);
                let home_row = row![register_button, monthly_summary_button].spacing(20);

                container(home_row)
                    .center_x(Fill)
                    .center_y(Fill)
                    .padding(20)
                    .into()
            }
            _ => {
                // Create the other screen views
                let back_button = button("Back").on_press(Message::GoToHome);
                let content = column![back_button];
                Element::new(content)
            }
        }
    }
}
