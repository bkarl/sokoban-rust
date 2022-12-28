pub const MAX_MAP_DIM: usize = 30;

use crate::{MoveDirection, movement};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    pub map: [[MapTile; MAX_MAP_DIM]; MAX_MAP_DIM],
    pub player_position: Position,
    pub movable_blocks: Vec<MovableBlock>,
    pub movable_blocks_in_final_position: u32,
    pub id: u32
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MapTile {
    Space,
    Wall,
    Block,
    TargetZone,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MovableBlock {
    pub position: Position,
}

impl MovableBlock {
    pub fn move_to(&mut self, move_direction: &MoveDirection)
    {
        self.position = movement::calc_new_position_after_movement(move_direction, &self.position);
    }
}

impl Map {
    pub fn parse_single_line(&mut self, line: &str, line_idx: usize) {
        for (idx, c) in line.chars().enumerate() {
            let mut tile = MapTile::Space;
            match c {
                'X' => tile = MapTile::Wall,
                '*' => self.movable_blocks.push(MovableBlock {position: Position {y: line_idx as i32, x: idx as i32}}),
                '.' => tile = MapTile::TargetZone,
                '@' => {
                    self.player_position = Position {
                        x: idx as i32,
                        y: line_idx as i32,
                    }
                }
                ' ' => (),
                _ => (),
            }
            self.map[line_idx][idx] = tile;
        }
    }

    pub fn parse_map_block(&mut self, input_map_block: &Vec<&str>) {
        for (line_idx, line) in input_map_block.iter().enumerate() {
            self.parse_single_line(line, line_idx);
        }
    }

    pub fn is_movable_block_at(&self, position: &Position) -> bool {
        for block in self.movable_blocks.iter() {
            if block.position == *position {
                return true;
            }
        }
        return false;
    }

    pub fn get_movable_block_at(&mut self, position: &Position) -> Option<&mut MovableBlock> {
        for block in self.movable_blocks.iter_mut() {
            if block.position == *position {
                return Some(block);
            }
        }
        None
    }

    pub fn get_tile_type_for_position(&self, position: &Position) -> MapTile {
        self.map[position.y as usize][position.x as usize]
    }

    pub fn new() -> Map {
        return Map { map: [[MapTile::Space; MAX_MAP_DIM]; MAX_MAP_DIM], player_position: Position {x: 0, y: 0}, movable_blocks: Vec::new(), movable_blocks_in_final_position:0, id:0};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_map() {
        let map_to_test = Map::new();
        assert_eq!(true, map_to_test.movable_blocks.is_empty());
        assert_eq!(0, map_to_test.movable_blocks_in_final_position);
        assert_eq!(0, map_to_test.id);
        for y in 0..MAX_MAP_DIM {
            for x in 0..MAX_MAP_DIM {
                assert_eq!(MapTile::Space, map_to_test.get_tile_type_for_position(&Position { x: x as i32, y: y as i32}));
            }
        }
    }

    #[test]
    fn test_is_movable_block_at() {
        let mut map = Map::new();
        map.movable_blocks.push(MovableBlock { position: Position { x: 5, y: 5 } });
        assert_eq!(true, map.is_movable_block_at(&Position { x: 5, y: 5 }));
    }

    #[test]
    fn test_parse_map_block() {
        let mut map = Map::new();
        let first_line = String::from(" X@*.");
        let second_line = String::from("X");
        let mut block_input: Vec<&str> = Vec::new();
        block_input.push(&first_line);
        block_input.push(&second_line);
        map.parse_map_block(&block_input);

        assert_eq!(true, map.is_movable_block_at(&Position { x: 3, y: 0 }));
        assert_eq!(
            MapTile::Space,
            map.get_tile_type_for_position(&Position { x: 0, y: 0 })
        );
        assert_eq!(
            MapTile::Wall,
            map.get_tile_type_for_position(&Position { x: 1, y: 0 })
        );
        assert_eq!(
            MapTile::Wall,
            map.get_tile_type_for_position(&Position { x: 0, y: 1 })
        );
        assert_eq!(
            MapTile::TargetZone,
            map.get_tile_type_for_position(&Position { x: 4, y: 0 })
        );
        assert_eq!(Position { x: 2, y: 0 }, map.player_position);
    }

    #[test]
    fn test_move_movable_block() {
        let mut block = MovableBlock{position: Position { x: 5, y: 6 }};
        block.move_to(&MoveDirection::Up);
        assert_eq!(Position { x: 5, y: 5 }, block.position);
    }

    #[test]
    fn test_get_movable_block_at_sucess() {
        let mut map = Map::new();
        let block = MovableBlock {position: Position {y: 7, x: 5}};
        map.movable_blocks.push(block);
        let result = map.get_movable_block_at(&Position{y: 7, x: 5});
        assert_eq!(result.unwrap(), &MovableBlock {position: Position {y: 7, x: 5}});
    }
    #[test]
    fn test_get_movable_block_at_failure() {
        let mut map = Map::new();
        let result = map.get_movable_block_at(&Position{x: 0, y: 0});
        assert_eq!(result.is_none(), true);
    }
}
