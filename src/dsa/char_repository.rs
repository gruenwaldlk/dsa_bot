use crate::dsa::character::Character;
use log::error;
use log::warn;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub(crate) struct CharRepository {
    pub(self) char_repository: HashMap<String, Character>,
    pub(self) id_repository: HashMap<u64, String>,
}

impl CharRepository {
    pub(crate) fn new_from_file(directory_path: &str) -> Self {
        let mut r = CharRepository::new();
        let dir_path = Path::new(directory_path);
        if dir_path.is_dir() {
            for e in fs::read_dir(&dir_path).unwrap() {
                let path = e.unwrap().path();
                let mut file = File::open(&path).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let c: Character = serde_json::from_str(&contents).unwrap();
                r.add_char(c);
            }
        }
        r
    }
    pub(crate) fn new() -> Self {
        CharRepository {
            char_repository: HashMap::new(),
            id_repository: HashMap::new(),
        }
    }
    pub(crate) fn get_char_by_player_id(&self, player_id: u64) -> Option<&Character> {
        let key_value_pair = match self.id_repository.get_key_value(&player_id) {
            Some(kvp) => kvp,
            None => {
                warn!("No character found for player id {}!", player_id);
                return None;
            }
        };
        let char_id = CharRepository::create_char_id(key_value_pair.1, *key_value_pair.0);
        self.get_char_by_id(&char_id)
    }
    pub(crate) fn get_char_by_id(&self, id: &str) -> Option<&Character> {
        match self.char_repository.get(id) {
            Some(c) => Some(c),
            None => {
                error!("No character found for id {}!", id);
                None
            }
        }
    }
    pub(crate) fn get_char_by_name_and_player_id(
        &self,
        character_name: &str,
        player_id: u64,
    ) -> Option<&Character> {
        self.char_repository
            .get(&CharRepository::create_char_id(character_name, player_id))
    }
    pub(self) fn add_char(&mut self, character: Character) {
        self.id_repository
            .insert(character.owner(), character.name().to_string());
        self.char_repository.insert(
            CharRepository::create_char_id(character.name(), character.owner()),
            character,
        );
    }
    pub(self) fn create_char_id(character_name: &str, player_id: u64) -> String {
        format!("{}-{}", character_name, player_id)
    }
}
