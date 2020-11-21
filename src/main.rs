use gtk;
use std::env;

mod dbus;
mod error;
mod sys;
mod ui;

fn main() {
    if env::args().len() == 2 && env::args().into_iter().nth(1).unwrap() == "daemon" {
        dbus::start().unwrap();
    }

    gtk::init().unwrap();

    ui::build_ui();
    gtk::main();
}
