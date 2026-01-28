use crate::home::room::Room;

pub mod room;

#[derive(Debug)]
pub struct Home {
    name: String,
    rooms: Vec<Room>,
}

impl Home {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: Vec::with_capacity(10),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.into()
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn room(&self, name_room: &str) -> Option<&Room> {
        self.rooms.iter().find(|r| r.name() == name_room)
    }

    pub fn room_mut(&mut self, name_room: &str) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|r| r.name() == name_room)
    }

    pub fn remove_room(&mut self, name_room: &str) {
        self.rooms.retain(|r| r.name() != name_room);
    }

    pub fn report(&self) {
        println!("Дом - {}", self.name);

        self.rooms.iter().for_each(|r| r.report());
    }
}
