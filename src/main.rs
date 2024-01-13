pub mod ui;
pub mod prelude;
pub mod image;

use prelude::*;

fn main() {
    let main_window = WindowDesc::new(ui::build_ui())
        .window_size((1200.0, 800.0))
        .title("Imagine");
    let initial_data = ();


    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}