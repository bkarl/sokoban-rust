use crate::{Map, MapTile, Position};

#[derive(Debug, Eq, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn calc_new_position_after_movement(dir: &MoveDirection, current_position: &Position) -> Position {
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;

    match dir {
        MoveDirection::Up => dy = -1,
        MoveDirection::Down => dy = 1,
        MoveDirection::Left => dx = -1,
        MoveDirection::Right => dx = 1,
    }

    let new_position = Position {
        x: current_position.x + dx,
        y: current_position.y + dy,
    };
    return new_position;
}

pub fn can_move_to(current_map: &Map, target_position: &Position, dir: &MoveDirection, probing_block: bool) -> bool {
    let mut tile_type = current_map.get_tile_type_for_position(target_position);
    if current_map.is_movable_block_at(target_position) {
        tile_type = MapTile::Block;
    }
    match tile_type {
        MapTile::Space | MapTile::TargetZone => return true,
        MapTile::Block => {
            if probing_block {
                return false;
            }
            let new_target_position = calc_new_position_after_movement(&dir, target_position);

            if can_move_to(current_map, &new_target_position, dir, true) {
                return true;
            } else {
                return false;
            }
        }
        _ => return false,
    }
}

#[cfg(test)]
mod tests {

    macro_rules! calc_new_position_after_movement_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let pos_in = Position{x: 5, y: 5};
                let (move_dir, expected) = $value;
                let pos_out = calc_new_position_after_movement(&move_dir, &pos_in);
                assert_eq!(expected, pos_out);
            }
        )*
        }
    }

    use super::*;
    use crate::map::MovableBlock;
    calc_new_position_after_movement_tests! {
        test_up: (MoveDirection::Up, Position{x : 5, y : 4}),
        test_down: (MoveDirection::Down, Position{x : 5, y : 6}),
        test_left: (MoveDirection::Left, Position{x : 4, y : 5}),
        test_right: (MoveDirection::Right, Position{x : 6, y : 5}),
    }

    #[test]
    fn test_can_move_to() {
        let map = Map::new();
        assert!(can_move_to(&map, &Position { x: 1, y: 0}, &MoveDirection::Right, false));    
    }
   
    #[test]
    fn test_can_not_move_to() {
        let mut map = Map::new();
        map.map[0][1] = MapTile::Wall;
        assert!(!can_move_to(&map, &Position { x: 1, y: 0}, &MoveDirection::Right, false));    
    }

    #[test]
    fn test_can_move_block() {
        let mut map = Map::new();
        map.movable_blocks.push(MovableBlock { position: Position {x: 1, y: 0} });
        assert!(can_move_to(&map, &Position { x: 1, y: 0}, &MoveDirection::Right, false));    
    }
   
    #[test]
    fn test_cannot_move_block_in_wall() {
        let mut map = Map::new();
        map.map[0][2] = MapTile::Wall;
        map.movable_blocks.push(MovableBlock { position: Position {x: 1, y: 0} });
        assert!(!can_move_to(&map, &Position { x: 1, y: 0}, &MoveDirection::Right, false));    
    }

    #[test]
    fn test_cannot_move_block_in_block() {
        let mut map = Map::new();
        map.movable_blocks.push(MovableBlock { position: Position {x: 1, y: 0} });
        map.movable_blocks.push(MovableBlock { position: Position {x: 2, y: 0} });
        assert!(!can_move_to(&map, &Position { x: 1, y: 0}, &MoveDirection::Right, false));    
    }
}
