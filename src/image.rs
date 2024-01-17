use std::path::Path;

use druid::{piet::{InterpolationMode, CoreGraphicsImage}, RenderContext, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, Rect, widget::{Controller, Axis}, Selector};
use ::image::open;

use crate::prelude::*;

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
    /// TODO: Zoom is centered on the mouse position
    fn add_zoom(&mut self, zoom_delta: f64, ctx: &mut EventCtx) {
        if self.zoom + zoom_delta < self.min_zoom {
            return;
        }
        let old_image_rect = self.get_rect();
        self.zoom += zoom_delta;
        let parent_size = ctx.size();
        let image_rect = self.get_rect();
        if parent_size.width >= image_rect.width() && parent_size.height >= image_rect.height() {
            // Center image
            self.center = image_rect.center().to_vec2();
        } else {
            // Add the half of the new size to the center
            let new_size = (image_rect.size() - old_image_rect.size()) / 2.0;
            self.center += new_size.to_vec2();
            println!("Center: {:?}", self.center);

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
pub struct ImageWidget {
    cached_image: Option<CoreGraphicsImage>,
}

impl ImageWidget {
    pub fn new() -> Self {
        Self {
            cached_image: None,
        }
    }
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

    fn layout(&mut self, _lay: &mut LayoutCtx, bc: &BoxConstraints, data: &ImageState, _: &Env) -> Size {
        // Max of parent size
        // WARNING: The parent is a Scroll widget, so the parent size is infinite
        
        let image_rect = data.get_rect();
        image_rect.size()
            
    }
    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &ImageState, _env: &Env) {
        let raw_image_data = data.image_buf.raw_pixels();
        let image = if let Some(image) = &self.cached_image {
            image.clone()
        } else {
            let image = ctx.make_image(data.image_buf.width(), data.image_buf.height(), raw_image_data, druid::piet::ImageFormat::RgbaSeparate).unwrap();
            self.cached_image = Some(image.clone());
            image.clone()
        };

        let image_rect = data.get_rect();
    
        // TODO: Add margin to image rect
        ctx.draw_image(&image, image_rect, InterpolationMode::Bilinear);
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut ImageState, _env: &Env) {
    
    }
}

pub fn load_and_convert_image(path: impl AsRef<Path>) -> ImageBuf {
    let image = open(path).unwrap().to_rgba8();
    let size = (image.width() as usize, image.height() as usize);
    ImageBuf::from_raw(image.into_raw(), druid::piet::ImageFormat::RgbaSeparate, size.0, size.1)
}
pub struct ImageView<T, W>
where
    T: ImageStateTrait,
    W: Widget<T>,
{
    inner: Scroll<T, W>,
}

impl<T, W> ImageView<T, W>
where
    T: ImageStateTrait + Data,
    W: Widget<T>,
{
    pub fn new(child: W) -> Self{
        Self {
            inner: Scroll::new(child).horizontal().vertical(),
        }
    }
}


impl<T, W> Widget<T> for ImageView<T, W>
where
    T: ImageStateTrait + Data,
    W: Widget<T>,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        
        match event {
            Event::Zoom(zoom_delta) => {
                data.add_zoom(*zoom_delta, ctx);
                let mut scroll_to = data.get_center();
                scroll_to -= ctx.size().to_vec2() / 2.0;
                self.inner.scroll_to_on_axis(ctx, Axis::Horizontal, scroll_to.x);
                self.inner.scroll_to_on_axis(ctx, Axis::Vertical, scroll_to.y);
                self.inner.event(ctx, event, data, env);
            },            
            Event::MouseMove(mouse_event) => {
                data.set_mouse_pos(mouse_event.pos.to_vec2());
                self.inner.event(ctx, event, data, env);
            },
            _ => {
                self.inner.event(ctx, event, data, env);
            }
        
        }
        // Update the center of the image if the scroll position changed
        let scroll_pos = self.inner.offset();
        let view_rect = self.inner.viewport_rect().size().to_vec2();
        let new_center = scroll_pos + view_rect / 2.0;
        println!("New center: {:?}", new_center);
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
        self.inner.paint(ctx, data, env);
    }

    
}

/*
impl<T, W: Widget<T>> Widget<T> for MyCustomScroll<W> {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                // Set initial scroll position
                self.inner.scroll_to(data.scroll_position);
            },
            _ => (),
        }

        self.inner.lifecycle(ctx, event, data, env);
    }

    // Implement other required methods...
}

pub struct ImageViewController {
    scroll_widget_id: WidgetId,
}

impl ImageViewController {
    pub fn new(scroll_widget_id: WidgetId) -> Self {
        Self { scroll_widget_id }
    }

    fn scroll_to_position(&self, ctx: &mut EventCtx, position: Vec2) {
       ctx.submit_command(SCROLL_TO.with(position).to(self.scroll_widget_id));
    }
}

impl<W: Widget<ImageState>> Controller<ImageState, W> for ImageViewController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut ImageState, env: &Env) {
       match event {
        Event::Zoom(zoom_delta) => {
            data.add_zoom(*zoom_delta, ctx);

            self.scroll_to_position(ctx, Vec2::new(0.0, 100.0));
            
        },
        Event::Command(cmd) if cmd.is(SCROLL_TO) => {
            let position = *cmd.get_unchecked(SCROLL_TO);
            // Implement the logic to scroll to the specified position
            
        },

        Event::MouseMove(mouse_event) => {
            data.mouse_pos = mouse_event.pos.to_vec2();
        },
        _ => {
            child.event(ctx, event, data, env);
        }
       
       }
    }

    // You can also override other methods like lifecycle, update, etc., as needed
}

 */