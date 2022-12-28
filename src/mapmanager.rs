use std::fs;
use regex::Regex;
use std::io;
use crate::map::Map;

const MAP_PATH : &str = "data/maps/maps.txt";

pub struct MapManager {
    pub maps: Vec<Map>,
}

pub struct DefaultMapContentProvider {}

impl MapContentProvider for DefaultMapContentProvider {
    fn get_maps(&self) -> Result<String,io::Error>
    {
        fs::read_to_string(MAP_PATH)
    }
}
pub trait MapContentProvider {
    fn get_maps(&self) -> Result<String,io::Error>;
}

impl MapManager {
    pub fn read_maps(&mut self, map_content_provider: impl MapContentProvider) -> Result<(), io::Error> {        
        let regex_mapcontent = Regex::new(r"^[ X]+[ X*@\.&]+").unwrap();
        let regex_divider = Regex::new(r"^\*+").unwrap();

        let map_contents = map_content_provider.get_maps()?;
        let mut currentmap = Map::new();
        let mut map_block : Vec<&str> = Vec::new();
        for line in map_contents.lines() {
            if regex_mapcontent.is_match(line) {
                map_block.push(line);
            } else if regex_divider.is_match(line) {
                currentmap.parse_map_block(&map_block);
                currentmap.id = self.maps.len() as u32;
                self.maps.push(currentmap);
                currentmap = Map::new();
                map_block.clear();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct FakeMapContentProvider {}

    impl MapContentProvider for FakeMapContentProvider {
    fn get_maps(&self) -> Result<String,io::Error>
    {
        Ok(String::from(
        "blablabla\n\
        blabla\n\
         XX\n\
        *************************************\n\
         XX\n\
        ******"))
    }
}

    #[test]
    fn test_map_manager() {
        let mut map_manager = MapManager{ maps : Vec::new()};
        let result = map_manager.read_maps(FakeMapContentProvider{});
        assert!(result.is_ok());
        assert_eq!(2, map_manager.maps.len());
        assert_eq!(1, map_manager.maps[1].id);
    }
}
