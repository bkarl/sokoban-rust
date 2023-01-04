use std::io;

use crate::input::{GameCommand, UserInputProvider};
use crate::{Map, MapTile, MapManager,MoveDirection, DefaultMapContentProvider, Position};
use crate::{PlatformSpecific, Draw};
use crate::movement;
pub struct Game {
    map_manager: MapManager,
    current_map_id: u32,
}

impl Game {

    pub fn new() -> Game {
        Game {map_manager: MapManager { maps: Vec::new() }, current_map_id: 0 }    
    }
    
    pub fn init(&mut self, platform: &PlatformSpecific) -> Result<(), io::Error> {
        self.map_manager.read_maps(DefaultMapContentProvider {})?;
        platform.renderer.setup();
        Ok(())
    }

    fn get_current_map(&self) -> Map {
        self.map_manager.maps[self.current_map_id as usize].clone()
    }

    pub fn main_loop(&mut self, platform: &mut PlatformSpecific) {
        while let Some(cmd) = self.input_loop(platform) {
            match cmd {
                GameCommand::Quit => break,
                GameCommand::NextMap => {
                    if !self.map_manager.maps.is_empty() && self.current_map_id + 1 < self.map_manager.maps.len() as u32 { 
                        self.current_map_id += 1
                    }}
                GameCommand::PreviousMap => {
                        if self.current_map_id > 0 {
                            self.current_map_id -= 1;
                }}
                _ => ()
            }
        }
    }

    fn render(&self, drawer : &mut Box<(dyn Draw)>, map: &Map) {
        drawer.draw(map);
    }

    fn input_loop(&self, platform: &mut PlatformSpecific) -> Option<GameCommand> {
        let mut current_map = self.get_current_map();
        self.render(&mut platform.renderer, &current_map);
        let mut user_input = platform.input_provider.get_user_input();
        while let Some(movedir) = user_input.movement_command {
            self.handle_movement(&mut current_map, movedir);
            self.render(&mut platform.renderer, &current_map);
            if self.check_has_won(&current_map) {
                return Some(GameCommand::NextMap);
            }
            user_input = platform.input_provider.get_user_input();
        }

        user_input.game_command
    }

    fn handle_movement(&self, current_map: &mut Map, movedir: MoveDirection) {
        let new_pos = movement::calc_new_position_after_movement(&movedir, &current_map.player_position);
        if movement::can_move_to(&current_map, &new_pos, &movedir, false) {
            if let Some(block) = current_map.get_movable_block_at(&new_pos) {
                let new_pos_block = movement::calc_new_position_after_movement(&movedir, &new_pos);
                block.move_to(&movedir);
                self.calc_nof_blocks_in_target_position(current_map, &new_pos, &new_pos_block);
            }
            current_map.player_position = new_pos;
        }
    }

    fn calc_nof_blocks_in_target_position(&self, map: &mut Map, old_position: &Position, new_position: &Position) {
        if map.get_tile_type_for_position(&new_position) == MapTile::TargetZone && map.get_tile_type_for_position(&old_position) != MapTile::TargetZone
        {
            map.movable_blocks_in_final_position += 1;
        }

        if map.get_tile_type_for_position(&new_position) != MapTile::TargetZone && map.get_tile_type_for_position(&old_position) == MapTile::TargetZone
        {
            map.movable_blocks_in_final_position -= 1;
        }
    }

    fn check_has_won(&self, map: &Map) -> bool {       
        if map.movable_blocks_in_final_position == map.movable_blocks.len() as u32 {
            return true;
        }
        return false;
    }

    pub fn tear_down(&self, platform: &PlatformSpecific) {
        platform.renderer.teardown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::MovableBlock;
    fn setup_tests() -> (Game, Map) {
        
        let mut game = Game::new(); 
        let mut map = Map::new();
        map.map[0][0] = MapTile::TargetZone;
        game.map_manager.maps.push(map.clone());
        (game, map)
    }

    #[test]
    fn test_move_box_into_target_zone() {
        let (game, mut map)  = setup_tests();
        let old_position = Position {x: 1, y: 0};        
        let new_position = Position {x: 0, y: 0};        
        game.calc_nof_blocks_in_target_position(&mut map, &old_position, &new_position);
        assert_eq!(1, map.movable_blocks_in_final_position);
    }

    
    #[test]
    fn test_move_box_out_of_target_zone() {
        let (game, mut map)  = setup_tests();
        let old_position = Position {x: 0, y: 0};        
        let new_position = Position {x: 1, y: 0};        
        map.movable_blocks_in_final_position = 1;
        game.calc_nof_blocks_in_target_position(&mut map, &old_position, &new_position);
        assert_eq!(0, map.movable_blocks_in_final_position);
    }

    #[test]
    fn test_move_box_in_target_zone() {       
        let (game, mut map)  = setup_tests();
        let old_position = Position {x: 0, y: 0};        
        let new_position = Position {x: 1, y: 0};        
        map.movable_blocks_in_final_position = 1;
        map.map[0][1] = MapTile::TargetZone;
        game.calc_nof_blocks_in_target_position(&mut map, &old_position, &new_position);
        assert_eq!(1, map.movable_blocks_in_final_position);
    }

    #[test]
    fn test_has_won() {       
        let (game, mut map)  = setup_tests();
        map.movable_blocks.push(MovableBlock { position: Position {x: 0, y: 0} });
        map.movable_blocks_in_final_position = 1;
        assert!(game.check_has_won(&map));
    }

    #[test]
    fn test_has_not_won() {       
        let (game, mut map)  = setup_tests();
        map.movable_blocks.push(MovableBlock { position: Position {x: 0, y: 0} });
        assert!(!game.check_has_won(&map));
    }

    #[test]
    fn test_get_current_map() {       
        let (game, _)  = setup_tests();
        let mut equal_map = Map::new();
        equal_map.map[0][0] = MapTile::TargetZone;
        assert_eq!(equal_map, game.get_current_map());
    }
}
