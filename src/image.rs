use std::path::Path;

use druid::{piet::InterpolationMode, LifeCycleCtx, LifeCycle, widget::{Axis, TextBox}, Affine};
use ::image::{open, ImageError};

#[cfg(target_os = "macos")]
use druid::piet::CoreGraphicsImage as CoreGraphicsImage;

#[cfg(not(target_os = "macos"))]
use druid::piet::CairoImage as CoreGraphicsImage;

use crate::prelude::*;

use self::delegate::CTRL;

#[derive(Clone, Data, Lens)]
pub struct ImageState {
    pub zoom: f64,
    pub min_zoom: f64,
    pub center: Vec2,
    pub image_buf: ImageBuf,
    pub mouse_pos: Vec2,
    pub path: String,
}

impl Default for ImageState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            center: Vec2::new(0.0, 0.0),
            mouse_pos: Vec2::new(0.0, 0.0),
            image_buf: ImageBuf::empty(),
            path: String::new(),
            min_zoom: 0.2,
        }
    }
}

pub trait ImageStateTrait {
    fn add_zoom(&mut self, zoom_delta: f64, ctx: &mut EventCtx);
    fn change_image(&mut self, path: &str, window_size: Size);
    fn get_rect(&self) -> druid::Rect;
    fn set_mouse_pos(&mut self, mouse_pos: Vec2);
    fn get_center(&self) -> Vec2;
    fn set_center(&mut self, center: Vec2);
}


impl ImageStateTrait for ImageState {
    /// Change the image and reset the zoom
    fn change_image(&mut self, path: &str, window_size: Size) {
        self.image_buf = load_and_convert_image(path);
        self.path = path.to_string();
        
        let image_rect = self.image_buf.size().to_rect();
        // Compute zoom to fit image in window
        let zoom_x = window_size.width / image_rect.width();
        let zoom_y = window_size.height / image_rect.height();
        self.zoom = zoom_x.min(zoom_y);
        self.center = image_rect.center().to_vec2();
        self.min_zoom = self.zoom / 5.0;
    }

    /// Get the rect of the image in the window (with the current zoom)
    fn get_rect(&self) -> druid::Rect {
        self.image_buf.size().to_rect().scale_from_origin(self.zoom)
    }

    /// Add a zoom delta to the current zoom
    /// Zoom is clamped between min_zoom and infinity
    /// Zoom is centered on the mouse position
    fn add_zoom(&mut self, zoom_delta: f64, ctx: &mut EventCtx) {
        if self.zoom + zoom_delta < self.min_zoom {
            return;
        }
        self.zoom += zoom_delta;
        let parent_size = ctx.size();
        let image_rect = self.get_rect();
        if parent_size.width >= image_rect.width() && parent_size.height >= image_rect.height() {
            // Center image
            self.center = image_rect.center().to_vec2();
        } else {
            // Mouse position in image coordinates
            let top_left = self.center - parent_size.to_vec2() / 2.0;
            let zoom_ratio = self.zoom / (self.zoom - zoom_delta);

            let mouse_pos = (top_left + self.mouse_pos) * zoom_ratio;
            
            self.center *= zoom_ratio;

            let zoom_ratio = zoom_delta / (self.zoom - zoom_delta);
            self.center += (mouse_pos - self.center) * zoom_ratio;
        }

        ctx.request_layout();
        ctx.request_paint();
    }

    fn set_mouse_pos(&mut self, mouse_pos: Vec2) {
        self.mouse_pos = mouse_pos;
    }

    fn get_center(&self) -> Vec2 {
        self.center
    }

    fn set_center(&mut self, center: Vec2) {
        self.center = center;
    }
}

#[derive(Default)]
pub struct ImageWidget {
    cached_image: Option<CoreGraphicsImage>,
    text: Vec<TextBox<String>>,
}

impl Widget<ImageState> for ImageWidget {
    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &ImageState, _: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, prev_data: &ImageState, new_data: &ImageState, _: &Env) {
        if prev_data.path != new_data.path {   
            self.cached_image = None;
            ctx.request_paint();
            ctx.request_layout();
        }

        if prev_data.zoom != new_data.zoom {
            ctx.request_layout();
        }
    }
        
    fn layout(&mut self, _lay: &mut LayoutCtx, _bc: &BoxConstraints, data: &ImageState, _: &Env) -> Size {                
        let image_rect = data.get_rect();
        image_rect.size()
    }
    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &ImageState, _env: &Env) {
        if let Some(cached_img) = self.cached_image.as_ref()  {
            let image_rect = data.get_rect();
            ctx.draw_image(cached_img, image_rect, InterpolationMode::Bilinear);
        } else {
            let image_rect = data.get_rect();
            let image_size = data.image_buf.size();
            let raw_image_data = data.image_buf.raw_pixels();
            let cached_img = ctx.make_image(image_size.width as usize, image_size.height as usize, raw_image_data, druid::piet::ImageFormat::RgbaSeparate).unwrap();
            ctx.draw_image(&cached_img, image_rect, InterpolationMode::Bilinear);
            self.cached_image = Some(cached_img);
        }
        
        // TODO: Add margin to image rect
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut ImageState, _env: &Env) {
    
    }
}

pub fn load_and_convert_image(path: impl AsRef<Path>) -> ImageBuf {
    let image = match open(path) {
        Ok(image) => image.to_rgba8(),
        Err(e) => {
            match e {
                ImageError::IoError(e) => eprintln!("Failed to open image: {}", e),
                ImageError::Decoding(e) => eprintln!("Failed to decode image: {}", e),
                _ => eprintln!("Failed to open image: {}", e),
            };
            return ImageBuf::empty();
        }
    };

    let size = (image.width() as usize, image.height() as usize);
    // TODO: Check if the image has the correct format
    ImageBuf::from_raw(image.into_raw(), druid::piet::ImageFormat::RgbaSeparate, size.0, size.1)
}
pub struct ImageView<T, W>
where
    T: ImageStateTrait,
    W: Widget<T>,
{
    inner: Scroll<T, W>,
    ctrl_pressed: bool,
}

impl<T, W> ImageView<T, W>
where
    T: ImageStateTrait + Data,
    W: Widget<T>,
{
    pub fn new(child: W) -> Self{
        Self {
            inner: Scroll::new(child).horizontal().vertical(),
            ctrl_pressed: false,
        }
    }
}


impl<T, W> Widget<T> for ImageView<T, W>
where
    T: ImageStateTrait + Data,
    W: Widget<T>,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        let mut zoomed = false;
        match event {
            Event::Zoom(zoom_delta) => {
                data.add_zoom(*zoom_delta, ctx);

                zoomed = true;
            },            
            Event::Command(cmd) => {
                if cmd.is(CTRL) {
                    self.ctrl_pressed = *cmd.get_unchecked(CTRL);
                }
            },
            Event::Wheel(wheel_event) => {
                if self.ctrl_pressed {
                    let zoom_delta = -wheel_event.wheel_delta.y * 0.001;
                    data.add_zoom(zoom_delta, ctx);
                    zoomed = true;
                } else {
                    self.inner.event(ctx, event, data, env);
                }
            },
            Event::MouseMove(mouse_event) => {
                data.set_mouse_pos(mouse_event.pos.to_vec2());
                self.inner.event(ctx, event, data, env);
            },
            _ => {
                self.inner.event(ctx, event, data, env);
            }
        }
        if zoomed {
            // Scroll to keep the mouse position in the same place
            let mut scroll_to: Vec2 = data.get_center();
            scroll_to -= ctx.size().to_vec2() / 2.0;
            self.inner.scroll_to_on_axis(ctx, Axis::Horizontal, scroll_to.x);
            self.inner.scroll_to_on_axis(ctx, Axis::Vertical, scroll_to.y);
            self.inner.event(ctx, event, data, env);
            
        }
        // Update the center of the image if the scroll position changed
        let scroll_pos = self.inner.offset();
        let view_rect = self.inner.viewport_rect().size().to_vec2();
        let new_center = scroll_pos + view_rect / 2.0;
        data.set_center(new_center);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn compute_max_intrinsic(
        &mut self,
        axis: druid::widget::Axis,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> f64 {
        match axis {
            druid::widget::Axis::Horizontal => self.layout(ctx, bc, data, env).width,
            druid::widget::Axis::Vertical => self.layout(ctx, bc, data, env).height,
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        // Draw inner widget centered in the viewport
        let image_rect = data.get_rect();
        let viewport_rect = self.inner.viewport_rect();
        
        ctx.with_save(|ctx| {
            // Center the non overflowing axis
            if image_rect.width() < viewport_rect.width() {
                let offset = (viewport_rect.width() - image_rect.width()) / 2.0;
                ctx.transform(Affine::translate((offset, 0.0)));
            }

            if image_rect.height() < viewport_rect.height() {
                let offset = (viewport_rect.height() - image_rect.height()) / 2.0;
                ctx.transform(Affine::translate((0.0, offset)));
            }
            self.inner.paint(ctx, data, env);
        });
    }
}
