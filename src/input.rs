use std::io::{self, Result};
use termion::event::*;
use termion::input::TermRead;

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

pub struct TerminalInputProvider {
    stdin: Box<(dyn Iterator<Item = Result<Key>>)>,
}

impl UserInputProvider for TerminalInputProvider {
    fn get_user_input(&mut self) -> InputAction {
        let c = self.stdin.next().unwrap();
        let movedir : Option<MoveDirection> = match c.as_ref().unwrap() {
            Key::Left => Some(MoveDirection::Left),
            Key::Right => Some(MoveDirection::Right),
            Key::Up => Some(MoveDirection::Up),
            Key::Down => Some(MoveDirection::Down),
            _ => None
        };

        let cmd = match c.unwrap() {
            Key::Char('q') | Key::Esc => Some(GameCommand::Quit),
            Key::Char('n') => Some(GameCommand::NextMap),
            Key::Char('p') => Some(GameCommand::PreviousMap),
            Key::Char('r') => Some(GameCommand::Reset),
            _ => None
        };
        InputAction { movement_command: movedir, game_command: cmd}
    }
}

impl TerminalInputProvider {
    pub fn new() -> TerminalInputProvider {
        TerminalInputProvider { stdin: Box::new(io::stdin().keys())}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct FakeUserInputProvider {
        pub key: Key,
    }

    impl Iterator for FakeUserInputProvider {
        type Item = Result<Key>;
        fn next(&mut self) -> Option<Self::Item> {
            Some(Ok(self.key))
        }
    }

    #[test]
    fn test_input() {
        let fake_input_provider = FakeUserInputProvider {
            key: Key::Char('q'),
        };
        let mut input_provider = TerminalInputProvider{stdin: Box::new(fake_input_provider)};
        assert!(input_provider.get_user_input().movement_command.is_none());
    }

    macro_rules! movement_input_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (action, key) = $value;
                let fake_input_provider = FakeUserInputProvider {
                    key: key
                };
                let mut input_provider = TerminalInputProvider{stdin: Box::new(fake_input_provider)};
                assert_eq!(
                    action,
                    input_provider.get_user_input()
                );
            }
        )*
        }
    }

    movement_input_tests! {
        test_up: (InputAction { movement_command: Some(MoveDirection::Up), game_command: None}, Key::Up),
        test_down: (InputAction { movement_command: Some(MoveDirection::Down), game_command: None}, Key::Down),
        test_left: (InputAction { movement_command: Some(MoveDirection::Left), game_command: None}, Key::Left),
        test_right: (InputAction { movement_command: Some(MoveDirection::Right), game_command: None}, Key::Right),
        test_q: (InputAction { movement_command: None, game_command: Some(GameCommand::Quit)}, Key::Char('q')),
        test_p: (InputAction { movement_command: None, game_command: Some(GameCommand::PreviousMap)}, Key::Char('p')),
        test_n: (InputAction { movement_command: None, game_command: Some(GameCommand::NextMap)}, Key::Char('n')),
        test_r: (InputAction { movement_command: None, game_command: Some(GameCommand::Reset)}, Key::Char('r')),
    }
}
