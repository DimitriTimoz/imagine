use druid::{piet::InterpolationMode, RenderContext, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, Size, Point};
use ::image::open;

use crate::prelude::*;

#[derive(Clone, Data, Lens)]
pub struct ImageState {
    pub zoom: f64,
    pub center: (i32, i32),
    pub image_buf: ImageBuf,
    pub mouse_pos: Point,
}


impl Default for ImageState {
    fn default() -> Self {
        Self {
            zoom: 2.0,
            center: (0, 0),
            mouse_pos: Point::new(0.0, 0.0),
            image_buf: load_and_convert_image("/Users/dimitri/Documents/image.png")
        }
    }
}
pub struct ImageWidget {

}

impl Widget<ImageState> for ImageWidget {
    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &ImageState, _: &Env) {}

    fn update(&mut self, _: &mut UpdateCtx, _: &ImageState, _: &ImageState, _: &Env) {}

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &ImageState, _: &Env) -> Size {
        bc.max()
    }
    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &ImageState, env: &Env) {
        let raw_image_data = data.image_buf.raw_pixels();
        let image = ctx.make_image(data.image_buf.width(), data.image_buf.height(), raw_image_data, druid::piet::ImageFormat::RgbaSeparate).unwrap();
        // Center image
        let center = (data.center.0 as f64, data.center.1 as f64);
        let center = Point::new(center.0, center.1);
        ctx.transform(druid::Affine::translate(center.to_vec2()));
        ctx.draw_image(&image, druid::Rect::new(0.0, 0.0, data.image_buf.width() as f64 * data.zoom, data.image_buf.height() as f64 * data.zoom), InterpolationMode::Bilinear);
    }

    fn event(&mut self, ctx: &mut druid::widget::prelude::EventCtx, event: &druid::widget::prelude::Event, data: &mut ImageState, env: &Env) {
        match event {
            Event::Zoom(zoom) => {
                // TODO: zoom to mouse position
                data.zoom += zoom;
                ctx.request_paint();
            },
            Event::MouseMove(mouse_event) => {
                data.mouse_pos = mouse_event.pos;
                ctx.request_paint();
            },
            Event::Wheel(wheel_event) => {
                data.center.0 -= wheel_event.wheel_delta.x as i32;
                data.center.1 -= wheel_event.wheel_delta.y as i32;
                ctx.request_paint();
            },
            _ => {}
        }
    }
}

pub fn load_and_convert_image(path: &str) -> ImageBuf {
    let image = open(path).unwrap().to_rgba8();
    let size = (image.width() as usize, image.height() as usize);
    ImageBuf::from_raw(image.into_raw(), druid::piet::ImageFormat::RgbaSeparate, size.0, size.1)
}
