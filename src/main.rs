use crate::app_state::AppState;

mod app_state;
mod message;
mod screen;

fn main() -> iced::Result {
    // iced::application(AppState::default,
    //                   AppState::update,
    //                   AppState::view,
    // )
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    iced::application(AppState::title, AppState::update, AppState::view)
        .centered()
        .run()
}
