#[derive(Debug, Clone)]
pub enum MainMessage {
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
