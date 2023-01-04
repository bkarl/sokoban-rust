use crate::{Draw, TerminalDrawer, input::UserInputProvider, input::TerminalInput};

pub struct PlatformSpecific {
    pub renderer: Box<(dyn Draw)>,
    pub input_provider: Box<(dyn UserInputProvider)>
}

impl PlatformSpecific {
    pub fn new_terminal_platform() -> PlatformSpecific {
        let default_user_input = Box::new(TerminalInput::new());
        let drawer = Box::new(TerminalDrawer::new());
        PlatformSpecific { renderer: drawer, input_provider: default_user_input}
    }
}
