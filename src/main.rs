pub mod ui;
pub mod prelude;
pub mod image;
pub mod delegate;
pub mod dialog;
pub mod background;
pub mod colors;

use std::sync::Arc;

use delegate::Delegate;

use prelude::*;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub image_state: image::ImageState,
    pub text: Arc<String>,
    pub window_size: Size,
}


fn main() {
    // TODO: load the window but don't show it until we have an image
    // TODO: ask for a file to open if none is provided
    let main_window = WindowDesc::new(ui::build_ui())
        .window_size((1200.0, 800.0))
        .menu(ui::make_menu)
        .transparent(true)
        .title("Imagine");
    let initial_data = AppState {
        image_state: image::ImageState::default(),
        text: Arc::new("Hello World!".to_string()),
        window_size: Size::new(1200.0, 800.0),
    };
    

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(initial_data)
        .expect("Failed to launch application");
}