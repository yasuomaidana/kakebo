#[derive(Debug, Clone)]
pub enum Message {
    GoToHome,
    GoToMonthlySummary,
    RegisterMessage(RegisterMessage),
    GoToRegister,
}

#[derive(Debug, Clone)]
pub enum RegisterMessage {
    Increment,
    Decrement,
}
