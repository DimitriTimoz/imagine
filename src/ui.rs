use druid::{WidgetExt, Menu, WindowId, FileDialogOptions, MenuItem, commands};

use crate::{prelude::*, AppState, dialog::open_image_dialog};

pub fn build_ui() -> impl Widget<AppState> {
    Container::new(
Scroll::new(
        Container::new(image::ImageWidget {})
                .center()
        )
    ).lens(AppState::image_state).center()
}

#[allow(unused_assignments)]
pub fn make_menu(_: Option<WindowId>, _state: &AppState, _: &Env) -> Menu<AppState> {
    let mut base: Menu<AppState> = Menu::empty();

    base = Menu::new(LocalizedString::new(""))
        .entry(
            Menu::new(LocalizedString::new("common-menu-file-menu"))
                .entry( MenuItem::new(LocalizedString::new("common-menu-file-open"))
                    .command(commands::SHOW_OPEN_PANEL.with(open_image_dialog()))
                )
        );

    base
}
