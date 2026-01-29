use crate::{
    devices::{Device, socket::Socket, thermometer::Thermometer},
    home::{Home, room::Room},
};

mod devices;
mod home;

pub trait Reportable {
    fn report(&self);
}

fn report<T: Reportable + ?Sized>(item: &T) {
    item.report();
}

pub fn app() {
    let t1 = Thermometer::new("Домашний");
    let s1 = Socket::new("Алиса");
    let s2 = Socket::new("Гирлянда");

    let dev1: Device = t1.into();
    let dev2: Device = s1.into();
    let dev3: Device = s2.into();

    let room1 = room! {
        "Зал",
        "Термометр в зале" => dev1,
        "Розетка в зале" => dev2
    };
    let room2 = room! {
        "Спальня",
        "Розетка в спальне" => dev3
    };
    let room3 = Room::new("Кухня");

    let mut home = Home::new("квартира 156");

    home.add_room(room1);
    home.add_room(room2);
    home.add_room(room3);

    home.report();

    println!();

    home.remove_room("Кухня");
    home.room_mut("Зал")
        .unwrap()
        .remove_device("Розетка в зале");

    match home.device_mut("Спальня", "Розетка в спальне") {
        Ok(d) => match d.as_socket_mut() {
            Some(s) => {
                s.turn_on();
                s.set_power(60.5);
            }
            None => eprintln!("Это все-таки не розетка!"),
        },
        Err(e) => eprintln!("Ошибка: {}", e),
    }

    report(&home);

    println!();

    match home.device_mut("Кухня", "Розетка на кухне") {
        Ok(d) => d.report(),
        Err(e) => eprintln!("Ошибка: {}", e),
    };
}
