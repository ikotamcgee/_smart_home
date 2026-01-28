use std::collections::HashMap;

use crate::devices::Device;

#[derive(Debug)]
pub struct Room {
    name: String,
    devices: HashMap<String, Device>,
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

    pub fn add_device(&mut self, name_device: &str, device: Device) {
        self.devices.insert(name_device.into(), device);
    }

    pub fn device(&self, name_device: &str) -> Option<&Device> {
        self.devices.get(name_device)
    }

    pub fn device_mut(&mut self, name_device: &str) -> Option<&mut Device> {
        self.devices.get_mut(name_device)
    }

    pub fn remove_device(&mut self, name_device: &str) {
        self.devices.remove(name_device);
    }

    pub fn report(&self) {
        println!("Комната - {}", self.name);

        self.devices.iter().for_each(|(_, d)| d.report());
    }
}
