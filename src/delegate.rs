use druid::{AppDelegate, DelegateCtx, Command, Target, Env, WindowId, commands::{OPEN_FILE, self}, FileDialogOptions, Handled, Size};

use crate::{AppState, image::ImageState};

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.image_state.change_image(file_info.path().to_str().unwrap(), Size::new(1200.0, 800.0));
            return Handled::Yes;
        } 

        Handled::No
    }
}
