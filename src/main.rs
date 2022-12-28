use std::io;

mod game;
use game::Game;
mod map;
use map::{Map, MapTile, Position};
mod mapmanager;
use mapmanager::{DefaultMapContentProvider, MapManager};
mod movement;
use movement::MoveDirection;
mod input;
mod drawing;
use drawing::{TerminalDrawer, Draw};
mod platform;
use platform::PlatformSpecific;

fn main() -> Result<(), io::Error> {
    let mut game = Game::new();
    let mut platform = PlatformSpecific::new_terminal_platform();
    game.init()?;

    game.main_loop(&mut platform);
    game.tear_down();
    Ok(())
}
