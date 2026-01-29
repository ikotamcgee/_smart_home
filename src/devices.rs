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

impl Device {
    pub fn as_thermometer(&self) -> Option<&Thermometer> {
        match self {
            Self::Thermometer(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_thermometer_mut(&mut self) -> Option<&mut Thermometer> {
        match self {
            Self::Thermometer(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_socket(&self) -> Option<&Socket> {
        match self {
            Self::Socket(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_socket_mut(&mut self) -> Option<&mut Socket> {
        match self {
            Self::Socket(t) => Some(t),
            _ => None,
        }
    }
}
