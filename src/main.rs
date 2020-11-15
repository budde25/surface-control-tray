use crate::error::{CliResult, ErrorKind, Result, ResultExt};
use gtk::prelude::*;
use gtk::{AboutDialog, CheckMenuItem, Menu, MenuItem, Orientation, Separator};
use libappindicator::*;

mod error;
mod sys;

fn main() {
    gtk::init().unwrap();

    let mut tray = AppIndicator::new("Surface Control Tray", "rust");

    let mut menu = Menu::new();
    let performance = MenuItem::with_label("Performance");
    let dgpu = MenuItem::with_label("Enable GPU");
    let latch = MenuItem::with_label("Detach");
    let about = MenuItem::with_label("About");
    let quit = MenuItem::with_label("Quit");

    let performance_menu = Menu::new();
    let performance_normal = CheckMenuItem::with_label("Normal");
    let performance_battery_saver = CheckMenuItem::with_label("Battery Saver");
    let performance_better_performance = CheckMenuItem::with_label("Better Performance");
    let performance_best_performance = CheckMenuItem::with_label("Best Performance");

    performance_menu.append(&performance_normal);
    performance_menu.append(&performance_battery_saver);
    performance_menu.append(&performance_better_performance);
    performance_menu.append(&performance_best_performance);
    performance.set_submenu(Some(&performance_menu));

    let seperator_1 = Separator::new(Orientation::Vertical);
    let seperator_2 = Separator::new(Orientation::Vertical);
    let menu_seperator_1 = MenuItem::new();
    let menu_seperator_2 = MenuItem::new();
    menu_seperator_1.add(&seperator_1);
    menu_seperator_2.add(&seperator_2);

    menu.append(&performance);
    menu.append(&dgpu);
    menu.append(&menu_seperator_1);
    menu.append(&latch);
    menu.append(&menu_seperator_2);
    menu.append(&about);
    menu.append(&quit);

    quit.connect_activate(|_| {
        gtk::main_quit();
    });

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["Ethan Budd"]);
        p.set_website_label(Some("Github"));
        p.set_website(Some("http://gtk-rs.org"));
        p.set_title("About!");
        p.show_all();
    });

    dgpu.connect_activate(|w| {
        let enabled = w.get_label().unwrap() == "Disable dGPU";
        w.set_label(if enabled {
            "Enable dGPU"
        } else {
            "Disable dGPU"
        });
        set_dgpu(enabled);
    });

    tray.set_status(AppIndicatorStatus::Active);
    tray.set_menu(&mut menu);
    menu.show_all();
    gtk::main();
}

fn set_dgpu(enabled: bool) {
    let set_ps = if enabled {
        sys::dgpu::PowerState::Off
    } else {
        sys::dgpu::PowerState::On
    };
    let dev = sys::dgpu::Device::open().unwrap();
    dev.set_power(set_ps).unwrap();
}
