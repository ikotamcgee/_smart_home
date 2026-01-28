use crate::devices::{Device, socket::Socket, thermometer::Thermometer};

mod devices;
mod home;

pub fn app() {
    let mut t1 = Thermometer::new("Домашний");
    t1.set_temperature(24.5);

    let mut s1 = Socket::new("Алиса");
    s1.set_power(60.0);

    let dev1 = Device::Thermometer(t1);
    let dev2 = Device::Socket(s1);

    dev1.report();
    dev2.report();
}
