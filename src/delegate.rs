use druid::{AppDelegate, DelegateCtx, Command, Target, Env, commands, Handled, Selector, keyboard_types::Key};

use crate::{prelude::*, dialog::open_image_dialog};

use self::image::ImageStateTrait;

pub const CTRL: Selector<bool> = Selector::new("custom.ctrl_pressed");

pub struct Delegate {
    window_size: Size,
}

impl Default for Delegate {
    fn default() -> Self {
        Self {
            window_size: Size::new(1.0, 1.0),
        }
    }
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            // Get the window size
            data.image_state.change_image(file_info.path().to_str().unwrap(), self.window_size);
            // Show the window now that we have an image
            return Handled::Yes;
        } 
        Handled::No
    }

    fn event(
            &mut self,
            ctx: &mut DelegateCtx,
            window_id: WindowId,
            event: Event,
            _data: &mut AppState,
            _env: &Env,
        ) -> Option<Event> {
            match &event {
                Event::WindowConnected => {
                    // Hide the window until we have an image
                    //ctx.submit_command(commands::HIDE_WINDOW.to(window_id));
                    ctx.submit_command(commands::SHOW_OPEN_PANEL.with(open_image_dialog()).to(window_id));
                    ctx.submit_command(commands::HIDE_WINDOW.to(window_id));
                    Some(event)
                },
                Event::WindowSize(size) => {
                    self.window_size = *size;
                    Some(event)
                },    
                Event::KeyDown(key_event) if key_event.key == druid::keyboard_types::Key::Control => {
                    ctx.submit_command(CTRL.with(true));
                    None
                },
                Event::KeyUp(key_event) if key_event.key == druid::keyboard_types::Key::Control => {
                    ctx.submit_command(CTRL.with(false));
                    None
                },
                _ => Some(event),
            }
            
    }
    
}
