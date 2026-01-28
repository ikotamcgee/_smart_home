pub struct Socket {
    is_on: bool,
    power: f64,
}

impl Socket {
    fn new() -> Self {
        Self {
            is_on: false,
            power: 0.0,
        }
    }

    fn turn_on(&mut self) {
        self.is_on = true
    }

    fn turn_off(&mut self) {
        self.is_on = false
    }

    fn is_on(&self) -> bool {
        self.is_on
    }

    fn set_power(&mut self, value: f64) {
        self.power = value
    }

    fn power(&self) -> f64 {
        if self.is_on { self.power } else { 0.0 }
    }
}
