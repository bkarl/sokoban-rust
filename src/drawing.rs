use crate::map::{Map, MapTile};
use std::io::{Stdout, stdout, Write};

use crossterm::{
     queue, style::Print, terminal, cursor
};

pub trait Draw {
    fn setup(&self);
    fn draw(&mut self, map : &Map);
    fn teardown(&self);
}

pub struct TerminalDrawer {
    stdout: Stdout,
}


impl TerminalDrawer {
    pub fn new() -> TerminalDrawer {
        TerminalDrawer { stdout: stdout() }
    }

    fn draw_help_text(&mut self, map : &Map) {
       let string_to_print = format!(
            "Map {}\r\nq - quit, r - reset, n - next map, p - previous map\r\n",
            map.id);
       queue!(self.stdout, Print(string_to_print)).unwrap();
    }
}

impl Draw for TerminalDrawer {
    fn setup(&self) {
        terminal::enable_raw_mode().unwrap();        
    }
    
	fn draw(&mut self, map : &Map) {
		
       queue!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        let mut map_dim = 0;
        for y in 0..map.map.len() {
            let mut current_line = String::new();
            for x in 0..map.map[y].len() {
                match map.map[y][x] {
                    MapTile::Wall => current_line += "X",
                    MapTile::TargetZone => current_line += ".",
                    _ => current_line += " ",
                }
            }
            queue!(
                self.stdout,
                cursor::MoveTo(1,(y+1) as u16),
                Print(&current_line)
            )
            .unwrap();
            if current_line.trim().is_empty() {
                map_dim = y;
                break;
            }
        }
        queue!(
            self.stdout,
            cursor::MoveTo(
                (map.player_position.x + 1) as u16,
                (map.player_position.y + 1) as u16
            ),
            Print("@".to_string())
        )
        .unwrap();
        for block in map.movable_blocks.iter() {
            queue!(
                self.stdout,
                cursor::MoveTo((block.position.x + 1) as u16, (block.position.y + 1) as u16),
                Print("*".to_string())
            )
            .unwrap();
        }
       queue!(
            self.stdout,
            cursor::MoveTo(0, (map_dim + 2) as u16))
        .unwrap();

        self.draw_help_text(map);
        self.stdout.flush().unwrap();
	}

    fn teardown(&self) {
        terminal::disable_raw_mode().unwrap();        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawing_can_be_instatiated() {
        let _drawing_module = TerminalDrawer::new();
    }
}
