use std::path::Path;

use druid::{piet::InterpolationMode, RenderContext, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx};
use ::image::open;

use crate::prelude::*;

#[derive(Clone, Data, Lens)]
pub struct ImageState {
    pub zoom: f64,
    pub center: Point,
    pub image_buf: ImageBuf,
    pub mouse_pos: Vec2,
    pub path: String,
}

impl Default for ImageState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            center: Point::new(0.0, 0.0),
            mouse_pos: Vec2::new(0.0, 0.0),
            image_buf: ImageBuf::empty(),
            path: String::new(),
        }
    }
}

impl ImageState {
    pub fn change_image(&mut self, path: &str, window_size: Size) {
        self.image_buf = load_and_convert_image(path);
        self.path = path.to_string();
        
        let image_rect = self.image_buf.size().to_rect();
        // Compute zoom to fit image in window
        let zoom_x = window_size.width / image_rect.width();
        let zoom_y = window_size.height / image_rect.height();
        self.zoom = zoom_x.min(zoom_y);
        self.center = Point::new(0.0, 0.0);
    }

    pub fn get_rect(&self) -> druid::Rect {
        self.image_buf.size().to_rect().scale_from_origin(self.zoom)
    }

    pub fn add_zoom(&mut self, zoom_delta: f64, ctx: &mut EventCtx) {
        self.zoom += zoom_delta;
        let parent_size = ctx.size();
        let image_rect = self.get_rect();
        if parent_size.width > image_rect.width() && parent_size.height > image_rect.height() {
            // Center image
            self.center = Point::new(0.0, 0.0);
        } else {
            // Zoom to mouse position
            let mouse_pos = self.mouse_pos;
            let mouse_pos = mouse_pos - Point::new(image_rect.width() / 2.0, image_rect.height() / 2.0).to_vec2();
            let mouse_pos = mouse_pos / self.zoom;
            let mouse_pos = mouse_pos * (self.zoom - zoom_delta);
            self.center += mouse_pos * 0.01;
        }
        ctx.request_layout();
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
        if prev_data.path != new_data.path {   
            ctx.request_layout();         
            ctx.request_paint();
        }
    }

    fn layout(&mut self, lay: &mut LayoutCtx, bc: &BoxConstraints, data: &ImageState, _: &Env) -> Size {
        // Max of parent size
        // WARNING: The parent is a Scroll widget, so the parent size is infinite
        bc.constrain(data.get_rect().size())
        
    }
    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &ImageState, env: &Env) {
        let raw_image_data = data.image_buf.raw_pixels();
        let image = ctx.make_image(data.image_buf.width(), data.image_buf.height(), raw_image_data, druid::piet::ImageFormat::RgbaSeparate).unwrap();

        let image_rect = data.get_rect();
        ctx.draw_image(&image, image_rect, InterpolationMode::Bilinear);
    }

    fn event(&mut self, ctx: &mut druid::widget::prelude::EventCtx, event: &druid::widget::prelude::Event, data: &mut ImageState, env: &Env) {
        match event {
            Event::Zoom(zoom) => {
                // TODO: zoom to mouse position
                data.add_zoom(*zoom, ctx);
            },
            Event::MouseMove(mouse_event) => {
                data.mouse_pos = mouse_event.pos.to_vec2();

            },
            Event::Wheel(wheel_event) => {
                data.move_image(wheel_event.wheel_delta, ctx);
            },
            _ => {}
        }
    }
}

pub fn load_and_convert_image(path: impl AsRef<Path>) -> ImageBuf {
    let image = open(path).unwrap().to_rgba8();
    let size = (image.width() as usize, image.height() as usize);
    ImageBuf::from_raw(image.into_raw(), druid::piet::ImageFormat::RgbaSeparate, size.0, size.1)
}
