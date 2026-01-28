use crate::{
    devices::{Device, socket::Socket, thermometer::Thermometer},
    home::Home,
};

mod devices;
mod home;

pub trait Reportable {
    fn report(&self);
}

fn report<T: Reportable>(item: &T) {
    item.report();
}

pub fn app() {}
