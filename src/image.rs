use druid::{piet::InterpolationMode, RenderContext, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, Size, Point, Vec2};
use ::image::open;

use crate::prelude::*;

#[derive(Clone, Data, Lens)]
pub struct ImageState {
    pub zoom: f64,
    pub center: Point,
    pub image_buf: ImageBuf,
    pub mouse_pos: Vec2,
}

impl Default for ImageState {
    fn default() -> Self {
        Self {
            zoom: 2.0,
            center: Point::new(0.0, 0.0),
            mouse_pos: Vec2::new(0.0, 0.0),
            image_buf: load_and_convert_image("/Users/dimitri/Documents/image.png")
        }
    }
}

impl ImageState {
    pub fn get_rect(&self) -> druid::Rect {
        druid::Rect::new(0.0, 0.0, self.image_buf.width() as f64, self.image_buf.height() as f64).scale_from_origin(self.zoom)
    }

    pub fn add_zoom(&mut self, zoom_delta: f64, ctx: &mut EventCtx) {
        self.zoom += zoom_delta;
        let parent_size = ctx.size();
        let image_rect = self.get_rect();
        if parent_size.width > image_rect.width() && parent_size.height > image_rect.height() {
            // Center image
            self.center = Point::new(0.0, 0.0);
        }
        ctx.request_paint();
    }

    pub fn move_image(&mut self, delta: Vec2, ctx: &mut EventCtx) {
        let parent_size = ctx.size();
        let image_rect = self.get_rect();
        if parent_size.width > image_rect.width() && parent_size.height > image_rect.height() {
            // Center image
            self.center = Point::new(0.0, 0.0);
            return;
        } 
        self.center += delta * self.zoom;
        ctx.request_paint();
    }
}
pub struct ImageWidget {

}

impl Widget<ImageState> for ImageWidget {
    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &ImageState, _: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, prev_data: &ImageState, new_data: &ImageState, _: &Env) {
    }

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &ImageState, _: &Env) -> Size {
        bc.max()
    }
    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &ImageState, env: &Env) {
        let raw_image_data = data.image_buf.raw_pixels();
        let image = ctx.make_image(data.image_buf.width(), data.image_buf.height(), raw_image_data, druid::piet::ImageFormat::RgbaSeparate).unwrap();

        let parent_size = ctx.size();
        let image_rect = druid::Rect::new(0.0, 0.0, data.image_buf.width() as f64, data.image_buf.height() as f64);
        let image_rect = image_rect.scale_from_origin(data.zoom);

        let center = Point::new(parent_size.width / 2.0, parent_size.height / 2.0);
        let center = center - data.center;
        let center = center - Point::new(image_rect.width() / 2.0, image_rect.height() / 2.0).to_vec2();


        ctx.transform(druid::Affine::translate(center));
        ctx.draw_image(&image, druid::Rect::new(0.0, 0.0, data.image_buf.width() as f64 * data.zoom, data.image_buf.height() as f64 * data.zoom), InterpolationMode::Bilinear);
    }

    fn event(&mut self, ctx: &mut druid::widget::prelude::EventCtx, event: &druid::widget::prelude::Event, data: &mut ImageState, env: &Env) {
        match event {
            Event::Zoom(zoom) => {
                // TODO: zoom to mouse position
                data.add_zoom(*zoom, ctx);
            },
            Event::MouseMove(mouse_event) => {
                //data.move_image(mouse_event.pos, ctx);
            },
            Event::Wheel(wheel_event) => {
                data.move_image(wheel_event.wheel_delta, ctx);
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
