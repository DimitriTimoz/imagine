use druid::WidgetPod;

use crate::prelude::*;

pub struct CustomBackgroundWidget<T, W> {
    // Background color
    background_color: druid::Color,
    child: WidgetPod<T, W>,
}

impl<T: druid::Data, W: Widget<T>> CustomBackgroundWidget<T, W> {
    pub fn new(child: W) -> Self {
        CustomBackgroundWidget {
            background_color: Color::Rgba32(0x020202D0),
            child: WidgetPod::new(child),
        }
    }
    
    pub fn background_color(mut self, color: druid::Color) -> Self {
        self.background_color = color;
        self
    }
}


impl<T: druid::Data, W: Widget<T>> Widget<T>  for CustomBackgroundWidget<T, W> {
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = ctx.size().to_rect();
        ctx.fill(rect, &self.background_color);

        self.child.paint(ctx, data, env);
    }

    

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, data, env);
        if !old_data.same(data) {
            ctx.request_paint(); 
        }
    
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let child_size = self.child.layout(ctx, &bc.loosen(), data, env);
        let my_size = bc.constrain(child_size); 
    
        let child_x = (my_size.width - child_size.width) / 2.0;
        let child_y = (my_size.height - child_size.height) / 2.0;
        let child_origin = Point::new(child_x, child_y);
    
        self.child.set_origin(ctx, child_origin);
    
        my_size
    }
    
}

