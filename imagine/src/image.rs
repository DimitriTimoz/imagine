use std::path::Path;

use druid::{piet::{InterpolationMode, Text}, LifeCycleCtx, LifeCycle, widget::{Axis, TextBox, SizedBox, Padding, BackgroundBrush}, Affine, Target, Rect, im::Vector, LensExt};
use ::image::{open, ImageError};

#[cfg(target_os = "macos")]
use druid::piet::CoreGraphicsImage as CoreGraphicsImage;

#[cfg(not(target_os = "macos"))]
use druid::piet::CairoImage as CoreGraphicsImage;

use crate::prelude::*;

use self::{delegate::{CTRL, SEND_OCR, RESET_OCR}, ocr::Ocr};



#[derive(Clone, Data, Lens)]
pub struct ImageState {
    pub zoom: f64,
    pub min_zoom: f64,
    pub center: Vec2,
    pub image_buf: Arc<ImageBuf>,
    pub mouse_pos: Vec2,
    pub path: String,
    //pub recognized_list: Vector<String>
}

impl Default for ImageState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            center: Vec2::new(0.0, 0.0),
            mouse_pos: Vec2::new(0.0, 0.0),
            image_buf: Arc::new(ImageBuf::empty()),
            path: String::new(),
            min_zoom: 0.2,
        }
    }
}

pub trait ImageStateTrait {
    fn add_zoom(&mut self, zoom_delta: f64, ctx: &mut EventCtx);
    fn change_image(&mut self, path: &str, window_size: Size, handle: druid::ExtEventSink);
    fn get_rect(&self) -> druid::Rect;
    fn set_mouse_pos(&mut self, mouse_pos: Vec2);
    fn get_center(&self) -> Vec2;
    fn set_center(&mut self, center: Vec2);
}


impl ImageStateTrait for ImageState {
    /// Change the image and reset the zoom
    fn change_image(&mut self, path: &str, window_size: Size, handle: druid::ExtEventSink) {
        self.image_buf = Arc::new(load_and_convert_image(path));
        self.path = path.to_string();
        
        let image_rect = self.image_buf.size().to_rect();
        // Compute zoom to fit image in window
        let zoom_x = window_size.width / image_rect.width();
        let zoom_y = window_size.height / image_rect.height();
        self.zoom = zoom_x.min(zoom_y);
        self.center = image_rect.center().to_vec2();
        self.min_zoom = self.zoom / 5.0;
        
        // Call asynchroneously the ocr
        let path = path.to_string();

        std::thread::spawn(move || {
            let ocr = Ocr::get_text(path);
            handle.submit_command(SEND_OCR, ocr, Target::Auto).expect("Failed to send OCR complete event");
        });
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

type TextField = String;

#[derive(Clone, Data, Lens)]
struct OcrText {
    text_fields: Vector<TextField>,
}
pub struct ImageWidget {
    cached_image: Option<CoreGraphicsImage>,
    text_boxes: Vec<Container<String>>,
}

impl Default for ImageWidget {
    fn default() -> Self {
        let text_box = TextBox::<String>::new().with_text_color(Color::TRANSPARENT)
        
        .background(BackgroundBrush::Color(Color::RED))
        ;
        Self { 
            cached_image: None,
            text_boxes: vec![text_box],
        }
    }
}

impl Widget<ImageState> for ImageWidget {
    fn lifecycle(&mut self, lc_ctx: &mut LifeCycleCtx, lc: &LifeCycle, data: &ImageState, env: &Env) {
        for text_box in &mut self.text_boxes {
            text_box.lifecycle(lc_ctx, lc, &"pomme".to_string(), env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, prev_data: &ImageState, new_data: &ImageState, _: &Env) {
        if prev_data.path != new_data.path {   
            self.cached_image = None;
            ctx.request_paint();
            ctx.request_layout();
        }

        if prev_data.zoom != new_data.zoom {
            ctx.request_layout();
        }

        if !prev_data.center.same(&new_data.center) {
            ctx.request_paint();
            ctx.request_layout();
        }
    }
        
    fn layout(&mut self, lay: &mut LayoutCtx, _bc: &BoxConstraints, data: &ImageState, env: &Env) -> Size {                
        let image_rect = data.get_rect();
        // Compute text boxes
        let mut text_boxes = Vec::new();
        for text_box in &mut self.text_boxes {
            let text_box_size = text_box.layout(lay, _bc, &"pomme".to_string(), env);
            let text_box_rect = Rect::from_points(Point::new(0.0, 0.0), Point::new(20.0, 10.0));
            text_boxes.push(text_box_rect);
        }
        image_rect.size()
    }
    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &ImageState, env: &Env) {
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

        // Draw text boxes
        println!("text boxes: {:?}", self.text_boxes.len());
        for text_box in self.text_boxes.iter_mut() {
            println!("paint text box");
            text_box.paint(ctx, &"pomme".to_string(), env);
        }

        // Draw ocr boxes
        /*if let Some(ocr) = self.ocr.as_ref() {
            for ocr_text_box in &ocr.content {
                // Draw boxe
                let mut box_rect = Rect::from_points(ocr_text_box.boxes[0], ocr_text_box.boxes[1]);
                for point in &ocr_text_box.boxes {
                    box_rect = box_rect.union(Rect::from_center_size(*point, (1.0, 1.0)));
                }
                // Adapt rect to zoom
                box_rect = box_rect.scale_from_origin(data.zoom);
                ctx.stroke(box_rect, &Color::RED, 1.0);
            }
        }*/
        
        // TODO: Add margin to image rect
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut ImageState, _env: &Env) {
        if let Event::Command(cmd) = event {
            if cmd.is(SEND_OCR) {
                //self.ocr = Some(cmd.get_unchecked(SEND_OCR).clone());

                ctx.request_paint();    
            } else if cmd.is(RESET_OCR) {
                //self.text_boxes.clear();
            }
        }
        for text_box in &mut self.text_boxes {
            text_box.event(ctx, event, &mut "pomme".to_string(), _env);
        }
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
                }
            },
            Event::MouseMove(mouse_event) => {
                data.set_mouse_pos(mouse_event.pos.to_vec2());
            },
            _ => {
            }
        }
        if zoomed {
            // Scroll to keep the mouse position in the same place
            let mut scroll_to: Vec2 = data.get_center();
            scroll_to -= ctx.size().to_vec2() / 2.0;

            self.inner.scroll_to_on_axis(ctx, Axis::Horizontal, scroll_to.x);
            self.inner.scroll_to_on_axis(ctx, Axis::Vertical, scroll_to.y);
            
        } else {
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
