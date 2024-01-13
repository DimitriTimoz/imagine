use druid::WidgetExt;

use crate::prelude::*;

pub fn build_ui() -> impl Widget<()> {
    Container::new(
        Flex::row()
            .with_flex_child(image::build_image_obj(), 1.0)
            .center()
    ).center()   
}
