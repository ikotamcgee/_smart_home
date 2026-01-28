pub struct Thermometer {
    temperature: f64,
}

impl Thermometer {
    fn new() -> Self {
        Self { temperature: 0.0 }
    }

    fn temperature(&self) -> f64 {
        self.temperature
    }

    fn set_temperature(&mut self, value: f64) {
        self.temperature = value
    }
}
