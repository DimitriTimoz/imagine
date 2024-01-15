use druid::{AppDelegate, DelegateCtx, Command, Target, Env, commands, Handled};

use crate::{prelude::*, dialog::open_image_dialog};

pub struct Delegate;

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
            data.image_state.change_image(file_info.path().to_str().unwrap(), Size::new(1200.0, 800.0));
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
            match event {
                Event::WindowConnected => {
                    // Hide the window until we have an image
                    ctx.submit_command(commands::HIDE_WINDOW.to(window_id));
                    ctx.submit_command(commands::SHOW_OPEN_PANEL.with(open_image_dialog()).to(window_id));
                    Some(event)
                }
                _ => Some(event),
            }
            
    }
}
