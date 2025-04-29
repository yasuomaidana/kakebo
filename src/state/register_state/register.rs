use crate::message::main_message::RegisterMessage;

#[derive(Default, Debug, Clone)]
pub struct RegisterState {
    pub count: u32,
}

impl RegisterState {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }

    pub fn update(&mut self, message: RegisterMessage) {
        match message {
            RegisterMessage::Increment => self.increment(),
            RegisterMessage::Decrement => self.decrement(),
        }
    }
}
