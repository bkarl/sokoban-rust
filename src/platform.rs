use std::io::stdout;
use termion::raw::IntoRawMode;
use crate::{Draw, TerminalDrawer, input::UserInputProvider, input::TerminalInputProvider};

pub struct PlatformSpecific {
    pub renderer: Box<(dyn Draw)>,
    pub input_provider: Box<(dyn UserInputProvider)>
}

impl PlatformSpecific {
    pub fn new_terminal_platform() -> PlatformSpecific {
        let default_user_input = Box::new(TerminalInputProvider::new());
        let stdout = stdout().into_raw_mode().unwrap();
        let drawer = Box::new(TerminalDrawer::new(stdout));
        PlatformSpecific { renderer: drawer, input_provider: default_user_input}
    }
}
