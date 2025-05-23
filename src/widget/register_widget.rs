use crate::message::main_message::{Message, RegisterMessage};
use crate::state::register_state::register::RegisterState;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill};

pub fn register_widget(state: &RegisterState) -> Element<Message> {
    // Create the register screen view
    let back_button = row![
        button("Back")
            .on_press(Message::GoToHome)
            .style(button::danger)
    ];
    let count_controller = row![
        button("+").on_press(Message::RegisterMessage(RegisterMessage::Increment)),
        text(format!("Count: {}", state.count)),
        button("-").on_press(Message::RegisterMessage(RegisterMessage::Decrement)),
    ]
    .align_y(iced::alignment::Vertical::Center)
    .spacing(10)
    .padding(10);
    let content = container(
        column![count_controller, back_button]
            .align_x(iced::alignment::Horizontal::Center)
            .spacing(10),
    )
    .center_x(Fill)
    .center_y(Fill)
    .padding(20);
    Element::new(content)
}
