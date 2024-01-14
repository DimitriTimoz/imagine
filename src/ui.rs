use druid::{WidgetExt, Menu, MenuItem, WindowId, Env, LocalizedString, platform_menus::mac::file};

use crate::{prelude::*, AppState};

pub fn build_ui() -> impl Widget<AppState> {
    Container::new(
        Flex::row()
            .with_flex_child(image::ImageWidget {}, 1.0)
            .center()
    ).lens(AppState::image_state).center()
}

#[allow(unused_assignments)]
pub fn make_menu(_: Option<WindowId>, _state: &AppState, _: &Env) -> Menu<AppState> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = Menu::new(LocalizedString::new(""))
            .entry(file::default())

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
