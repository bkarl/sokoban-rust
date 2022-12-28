use crate::map::{Map, MapTile};
use std::io::Write;

pub trait Draw {
    fn draw(&mut self, map : &Map);
}

pub struct TerminalDrawer {
	stdout : termion::raw::RawTerminal<std::io::Stdout>
}

impl TerminalDrawer {
    pub fn new(stdout: termion::raw::RawTerminal<std::io::Stdout>) -> TerminalDrawer {
        TerminalDrawer { stdout: stdout}
    }

    fn draw_help_text(&mut self, map : &Map) {
       
       write!(
            self.stdout,
            "Map {}\r\nq - quit, r - reset, n - next map, p - previous map\r\n",
            map.id)
        .unwrap();
    }
}

impl Draw for TerminalDrawer {
	fn draw(&mut self, map : &Map) {
		
        write!(self.stdout, "{}", termion::clear::All).unwrap();
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
            write!(
                self.stdout,
                "{}{}\n",
                termion::cursor::Goto(1, (y + 1) as u16),
                current_line
            )
            .unwrap();
            if current_line.trim().is_empty() {
                map_dim = y;
                break;
            }
        }
        write!(
            self.stdout,
            "{}{}\n",
            termion::cursor::Goto(
                (map.player_position.x + 1) as u16,
                (map.player_position.y + 1) as u16
            ),
            "@"
        )
        .unwrap();
        for block in map.movable_blocks.iter() {
            write!(
                self.stdout,
                "{}{}\n",
                termion::cursor::Goto((block.position.x + 1) as u16, (block.position.y + 1) as u16),
                "*"
            )
            .unwrap();
        }
       write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(0, (map_dim + 2) as u16))
        .unwrap();

        self.draw_help_text(map);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::stdout;
    use termion::raw::IntoRawMode;

    #[test]
    fn test_drawing_can_be_instatiated() {
        let stdout_for_drawing = stdout().into_raw_mode().unwrap();
        let _drawing_module = TerminalDrawer{ stdout: stdout_for_drawing};
        stdout().into_raw_mode().unwrap().suspend_raw_mode().unwrap_or(()); 
    }
}
