//! # MenuBar Sample
//!
//! This sample demonstrates how to use Menus/MenuBars and MenuItems in Windows.
//!
//! /!\ This is different from the system menu bar (which are preferred) available in `gio::Menu`!

use gio;
use glib;
use gtk;
use libappindicator;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, CheckMenuItem, IconSize, Image, Label,
    Menu, MenuBar, MenuItem, WindowPosition,
};
use libappindicator::*;

fn main() {
    gtk::init();

    let mut tray = AppIndicator::new("Surface Control Tray", "rust");

    let mut menu = Menu::new();
    let about = MenuItem::with_label("About");
    let quit = MenuItem::with_label("Quit");

    let performance = MenuItem::with_label("Performance");
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
    menu.append(&performance);

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
    // check_item.connect_toggled(|w| {
    //     w.set_label(if w.get_active() {
    //         "Checked"
    //     } else {
    //         "Unchecked"
    //     });
    // });

    tray.set_status(AppIndicatorStatus::Active);
    tray.set_menu(&mut menu);
    menu.show_all();
    gtk::main();
}
