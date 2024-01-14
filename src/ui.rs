use druid::{WidgetExt, Menu, WindowId, platform_menus::mac::file, widget::{TextBox, BackgroundBrush, Scroll}, FileDialogOptions, MenuItem, commands};

use crate::{prelude::*, AppState};

pub fn build_ui() -> impl Widget<AppState> {
    Container::new(
Scroll::new(
        Flex::row()
                    .with_flex_child(image::ImageWidget {}, 1.0)
                .center()
        )
    ).lens(AppState::image_state).center()
}

#[allow(unused_assignments)]
pub fn make_menu(_: Option<WindowId>, _state: &AppState, _: &Env) -> Menu<AppState> {
    let mut base: Menu<AppState> = Menu::empty();
    let open_dialog_options = FileDialogOptions::new()
        .name_label("Open")
        .title("Open")
        .button_text("Open");


    #[cfg(target_os = "macos")]
    {
        base = Menu::new(LocalizedString::new(""))
            .entry(
                Menu::new(LocalizedString::new("common-menu-file-menu"))
                    .entry( MenuItem::new(LocalizedString::new("common-menu-file-open"))
                        .command(commands::SHOW_OPEN_PANEL.with(open_dialog_options))
                    )
            )


    }
    #[cfg(any(
        target_os = "windows",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd"
    ))]
    {
        base = base.entry(druid::platform_menus::win::file::default());
    }

    base
}
