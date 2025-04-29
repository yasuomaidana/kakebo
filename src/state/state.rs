use crate::message::main_message::MainMessage;
use iced::Element;

pub(crate) trait State: Default + StateMethods {}

pub(crate) trait StateMethods {
    fn title(&self) -> String;
    fn update(&mut self, message: MainMessage);
    fn view(&self) -> Element<MainMessage>;
}
