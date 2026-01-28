use crate::devices::Device;

pub struct Room {
    name: String,
    devices: Vec<Device>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            devices: Vec::with_capacity(10),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.into()
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn device(&self, name_device: &str) -> Option<&Device> {
        self.devices.iter().find(|d| d.name() == name_device)
    }

    pub fn device_mut(&mut self, name_device: &str) -> Option<&mut Device> {
        self.devices.iter_mut().find(|d| d.name() == name_device)
    }

    pub fn remove_device(&mut self, name_device: &str) {
        self.devices.retain(|d| d.name() != name_device);
    }

    pub fn report(&self) {
        println!("Комната - {}", self.name);

        self.devices.iter().for_each(|d| d.report());
    }
}
