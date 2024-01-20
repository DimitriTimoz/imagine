pub mod ui;
pub mod prelude;
pub mod image;
pub mod delegate;
pub mod dialog;
pub mod background;
pub mod colors;
pub mod ocr;


use delegate::Delegate;

use prelude::*;


#[derive(Clone, Data, Lens, Default)]
pub struct KeyState {
    pub ctrl: bool,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub image_state: image::ImageState,
    pub text: Arc<String>,
    pub key_state: KeyState,
}


fn main() {
    // TODO: load the window but don't show it until we have an image
    // TODO: ask for a file to open if none is provided
    let main_window = WindowDesc::new(ui::build_ui())
        .window_size((1200.0, 800.0))
        .menu(ui::make_menu)
        .title("Imagine");

    #[cfg(target_os = "macos")]
    let main_window = main_window.transparent(true);
    let initial_data = AppState {
        image_state: image::ImageState::default(),
        text: Arc::new("Hello World!".to_string()),
        key_state: KeyState::default(),
    };
    

    AppLauncher::with_window(main_window)
        .delegate(Delegate::default())
        .log_to_console()
        .launch(initial_data)
        .expect("Failed to launch application");
}