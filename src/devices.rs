use crate::{
    Reportable,
    devices::{socket::Socket, thermometer::Thermometer},
};

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

impl Reportable for Device {
    fn report(&self) {
        match self {
            Self::Thermometer(t) => t.report(),
            Self::Socket(s) => s.report(),
        }
    }
}
