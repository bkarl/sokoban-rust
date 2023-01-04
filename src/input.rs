use crossterm::{
    event::{
        read, Event, KeyCode, KeyEvent},
    Result,
};
use crate::MoveDirection;

#[derive(Debug, Eq, PartialEq)]
pub enum GameCommand {
    Reset,
    Quit,
    NextMap,
    PreviousMap
}

#[derive(Debug, Eq, PartialEq)]
pub struct InputAction {
    pub movement_command: Option<MoveDirection>,
    pub game_command: Option<GameCommand>
}

pub trait UserInputProvider {
    fn get_user_input(&mut self) -> InputAction;
}


impl<S: UserInputProvider + ?Sized> UserInputProvider for Box<S> {
    fn get_user_input(&mut self) -> InputAction {
        (**self).get_user_input()
    }
}

pub trait TerminalInputProvider {
    fn read_key_input(&self) -> Result<Event>;
}

pub struct CrosstermInput {
}

impl TerminalInputProvider for CrosstermInput {
    fn read_key_input(&self) -> Result<Event> {
        read()
    }
}

pub struct TerminalInput {
    input_provider : Box<dyn TerminalInputProvider>,
}

impl UserInputProvider for TerminalInput {
    fn get_user_input(&mut self) -> InputAction {
        let event = self.input_provider.read_key_input();
        let movedir : Option<MoveDirection> = match event.as_ref().unwrap() {
            Event::Key(KeyEvent{code: KeyCode::Left, ..}) => Some(MoveDirection::Left),
            Event::Key(KeyEvent{code: KeyCode::Right, ..}) => Some(MoveDirection::Right),
            Event::Key(KeyEvent{code: KeyCode::Up, ..}) => Some(MoveDirection::Up),
            Event::Key(KeyEvent{code: KeyCode::Down, ..}) => Some(MoveDirection::Down),
            _ => None
        };

        let cmd = match event.unwrap() {
            Event::Key(KeyEvent{code: KeyCode::Char('q') | KeyCode::Esc, ..}) => Some(GameCommand::Quit),
            Event::Key(KeyEvent{code: KeyCode::Char('n'), ..}) => Some(GameCommand::NextMap),
            Event::Key(KeyEvent{code: KeyCode::Char('p'), ..}) => Some(GameCommand::PreviousMap),
            Event::Key(KeyEvent{code: KeyCode::Char('r'), ..}) => Some(GameCommand::Reset),
            _ => None
        };
        InputAction { movement_command: movedir, game_command: cmd}
    }
}

impl TerminalInput {
    pub fn new() -> TerminalInput {
        TerminalInput { input_provider: Box::new(CrosstermInput {})}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct FakeInput {
        pub key: crossterm::event::Event,
    }

    impl TerminalInputProvider for FakeInput{
        fn read_key_input(&self) -> Result<Event> {
            Ok(self.key)
        }
    }
   

    #[test]
    fn test_input() {
        let fake_input_provider = FakeInput{
            key: Event::Key(KeyCode::Char('q').into()),
        };
        let mut input_provider = TerminalInput{input_provider: Box::new(fake_input_provider)};
        assert!(input_provider.get_user_input().movement_command.is_none());
    }

    macro_rules! movement_input_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (action, key) = $value;
                let fake_input_provider = FakeInput{
                    key: key
                };
                let mut input_provider = TerminalInput{input_provider: Box::new(fake_input_provider)};
                assert_eq!(
                    action,
                    input_provider.get_user_input()
                );
            }
        )*
        }
    }

    movement_input_tests! {
        test_up: (InputAction { movement_command: Some(MoveDirection::Up), game_command: None}, Event::Key(KeyCode::Up.into())),
        test_down: (InputAction { movement_command: Some(MoveDirection::Down), game_command: None}, Event::Key(KeyCode::Down.into())),
        test_left: (InputAction { movement_command: Some(MoveDirection::Left), game_command: None}, Event::Key(KeyCode::Left.into())),
        test_right: (InputAction { movement_command: Some(MoveDirection::Right), game_command: None}, Event::Key(KeyCode::Right.into())),
        test_q: (InputAction { movement_command: None, game_command: Some(GameCommand::Quit)}, Event::Key(KeyCode::Char('q').into())),
        test_p: (InputAction { movement_command: None, game_command: Some(GameCommand::PreviousMap)}, Event::Key(KeyCode::Char('p').into())),
        test_n: (InputAction { movement_command: None, game_command: Some(GameCommand::NextMap)}, Event::Key(KeyCode::Char('n').into())),
        test_r: (InputAction { movement_command: None, game_command: Some(GameCommand::Reset)}, Event::Key(KeyCode::Char('r').into())),
    }
}
