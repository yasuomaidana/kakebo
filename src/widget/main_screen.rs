use crate::message::main_message::MainMessage;
use crate::state::register_state::register::RegisterState;
use crate::widget::main_screen::MainScreen::RegisterWidget;
use crate::widget::register_widget::register_widget;
use iced::widget::{button, container, row};
use iced::{Element, Fill};

#[derive(Debug)]
pub enum MainScreen {
    Home,
    RegisterWidget(RegisterState),
    MonthlySummary,
}

impl MainScreen {
    pub fn title(&self) -> String {
        format!(
            "{}",
            match self {
                MainScreen::Home => "Home",
                RegisterWidget(_) => "Register",
                MainScreen::MonthlySummary => "Monthly Summary",
            }
        )
    }
    pub(crate) fn view(&self) -> Element<MainMessage> {
        match self {
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
            RegisterWidget(state) => register_widget(state),

            _ => {
                // Create the other screen views
                let back_button = button("Back").on_press(MainMessage::GoToHome);
                let content = iced::widget::column![back_button];
                Element::new(content)
            }
        }
    }
}
