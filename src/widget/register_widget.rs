use crate::message::main_message::{MainMessage, RegisterMessage};
use crate::state::register_state::register::RegisterState;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill};

pub fn register_widget(state: &RegisterState) -> Element<MainMessage> {
    // Create the register screen view
    let back_button = button("Back").on_press(MainMessage::GoToHome);
    let count_controller = row![
        button("+").on_press(MainMessage::RegisterMessage(RegisterMessage::Increment)),
        text(format!("Count: {}", state.count)),
        button("-").on_press(MainMessage::RegisterMessage(RegisterMessage::Decrement)),
    ]
    .spacing(10)
    .padding(10);
    let content = container(column![count_controller, back_button])
        .center_x(Fill)
        .center_y(Fill)
        .padding(20);
    Element::new(content)
}
