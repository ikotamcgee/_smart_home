#[derive(Debug)]
pub struct Thermometer {
    name: String,
    temperature: f64,
}

impl Thermometer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            temperature: 0.0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.into()
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    pub fn set_temperature(&mut self, value: f64) {
        self.temperature = value
    }
}
