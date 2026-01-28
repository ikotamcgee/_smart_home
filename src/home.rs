use std::collections::HashMap;

use crate::{Reportable, devices::Device, home::room::Room};

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

    pub fn add_room(&mut self, room_name: &str, room: Room) {
        self.rooms.insert(room_name.into(), room);
    }

    pub fn room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }

    pub fn room_mut(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_name)
    }

    pub fn remove_room(&mut self, room_name: &str) {
        self.rooms.remove(room_name);
    }

    pub fn device(&self, room_name: &str, device_name: &str) -> Result<&Device, HomeError> {
        let room = self.room(room_name).ok_or_else(|| {
            HomeError::RoomNotFound(format!("Комната с таким именем {} не найдена.", room_name))
        })?;

        let device = room.device(device_name).ok_or_else(|| {
            HomeError::DeviceNotFound(format!("Девайс с таким именем {} не найден.", device_name))
        })?;

        Ok(device)
    }
}

impl Reportable for Home {
    fn report(&self) {
        println!("Дом - {}", self.name);

        self.rooms.iter().for_each(|(_, r)| r.report());
    }
}
