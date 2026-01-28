use std::collections::HashMap;

use crate::{devices::Device, home::room::Room};

pub mod room;

#[derive(Debug)]
pub enum HomeError {
    DeviceNotFound(String),
    RoomNotFound(String),
}

impl std::fmt::Display for HomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for HomeError {}

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

    pub fn add_room(&mut self, name_room: &str, room: Room) {
        self.rooms.insert(name_room.into(), room);
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

    pub fn device(&self, name_room: &str, name_device: &str) -> Result<&Device, HomeError> {
        let room = self.room(name_room).ok_or_else(|| {
            HomeError::RoomNotFound(format!("Комната с таким именем {} не найдена.", name_room))
        })?;

        let device = room.device(name_device).ok_or_else(|| {
            HomeError::DeviceNotFound(format!("Девайс с таким именем {} не найден.", name_device))
        })?;

        Ok(device)
    }

    pub fn report(&self) {
        println!("Дом - {}", self.name);

        self.rooms.iter().for_each(|(_, r)| r.report());
    }
}
