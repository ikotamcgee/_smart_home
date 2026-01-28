pub struct Socket {
    name: String,
    is_on: bool,
    power: f64,
}

impl Socket {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            is_on: false,
            power: 0.0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.into()
    }

    pub fn turn_on(&mut self) {
        self.is_on = true
    }

    pub fn turn_off(&mut self) {
        self.is_on = false
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn set_power(&mut self, value: f64) {
        self.power = value
    }

    pub fn power(&self) -> f64 {
        if self.is_on { self.power } else { 0.0 }
    }
}
