mod devices;
mod home;

pub trait Reportable {
    fn report(&self);
}

pub fn app() {}
