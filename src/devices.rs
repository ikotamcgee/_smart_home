use crate::devices::{socket::Socket, thermometer::Thermometer};

pub mod socket;
pub mod thermometer;

pub enum Device {
    Thermometer(Thermometer),
    Socket(Socket),
}

impl Device {
    fn report(&self) {
        match self {
            Self::Thermometer(t) => {
                println!(
                    "Термометр {}\n
                    Температура: {}",
                    t.name(),
                    t.temperature()
                );
            }
            Self::Socket(s) => {
                println!(
                    "Розетка {}\n
                    Состояние: {}\n
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
}
