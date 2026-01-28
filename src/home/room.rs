use std::collections::HashMap;

use crate::{Reportable, devices::Device};

#[derive(Debug)]
pub struct Room {
    name: String,
    devices: HashMap<String, Device>,
}

#[macro_export]
macro_rules! room {
    ($room_name:expr, $($device_name:expr => $device:expr),* $(,)?) => {
        {
            let mut r = crate::home::room::Room::new($room_name);

            $(
                r.add_device($device_name, $device.into());
            )*

            r
        }
    };
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::with_capacity(10),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.into()
    }

    pub fn add_device(&mut self, device_name: &str, device: Device) {
        self.devices.insert(device_name.into(), device);
    }

    pub fn device(&self, device_name: &str) -> Option<&Device> {
        self.devices.get(device_name)
    }

    pub fn device_mut(&mut self, device_name: &str) -> Option<&mut Device> {
        self.devices.get_mut(device_name)
    }

    pub fn remove_device(&mut self, device_name: &str) {
        self.devices.remove(device_name);
    }
}

impl Reportable for Room {
    fn report(&self) {
        println!("Комната - {}", self.name);

        self.devices.iter().for_each(|(_, d)| d.report());
    }
}
