use crate::devices::{socket::Socket, thermometer::Thermometer};

pub mod socket;
pub mod thermometer;

#[derive(Debug)]
pub enum Device {
    Thermometer(Thermometer),
    Socket(Socket),
}

impl From<Thermometer> for Device {
    fn from(value: Thermometer) -> Self {
        Device::Thermometer(value)
    }
}

impl From<Socket> for Device {
    fn from(value: Socket) -> Self {
        Device::Socket(value)
    }
}

impl Device {
    pub fn report(&self) {
        match self {
            Self::Thermometer(t) => {
                println!(
                    "Термометр: {}\n\
                    Температура: {}",
                    t.name(),
                    t.temperature()
                );
            }
            Self::Socket(s) => {
                println!(
                    "Розетка: {}\n\
                    Состояние: {}\n\
					Текущая мощность: {}",
                    s.name(),
                    if s.is_on() {
                        "включена"
                    } else {
                        "выключена"
                    },
                    s.power()
                );
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Socket(s) => s.name(),
            Self::Thermometer(t) => t.name(),
        }
    }
}
