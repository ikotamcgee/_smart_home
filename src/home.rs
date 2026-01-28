use std::collections::HashMap;

use crate::home::room::Room;

pub mod room;

#[derive(Debug)]
pub struct Home {
    name: String,
    rooms: HashMap<String, Room>,
}

impl Home {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::with_capacity(10),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.into()
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name().into(), room);
    }

    pub fn room(&self, name_room: &str) -> Option<&Room> {
        self.rooms.get(name_room)
    }

    pub fn room_mut(&mut self, name_room: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name_room)
    }

    pub fn remove_room(&mut self, name_room: &str) {
        self.rooms.remove(name_room);
    }

    pub fn report(&self) {
        println!("Дом - {}", self.name);

        self.rooms.iter().for_each(|(_, r)| r.report());
    }
}
