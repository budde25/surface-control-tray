use crate::error::{CliResult, ErrorKind, Result, ResultExt};
use glib::Receiver;
use gtk::prelude::*;
use gtk::{AboutDialog, CheckMenuItem, Menu, MenuItem, Orientation, Separator};
use libappindicator::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::process::Command;
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::Duration;

mod error;
mod sys;

fn main() {
    gtk::init().unwrap();

    let mut tray = AppIndicator::new("Surface Control Tray", "");

    let mut menu = Menu::new();
    let performance = MenuItem::with_label("Performance");
    let dgpu = MenuItem::with_label("Enable GPU");
    let latch = MenuItem::with_label("Detach");
    let about = MenuItem::with_label("About");
    let quit = MenuItem::with_label("Quit");

    let performance_menu = Menu::new();
    let performance_normal = CheckMenuItem::with_label("Normal");
    let performance_battery_saver = CheckMenuItem::with_label("Battery Saver");
    let performance_better = CheckMenuItem::with_label("Better Performance");
    let performance_best = CheckMenuItem::with_label("Best Performance");

    performance_menu.append(&performance_normal);
    performance_menu.append(&performance_battery_saver);
    performance_menu.append(&performance_better);
    performance_menu.append(&performance_best);
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

    performance_normal.connect_activate(|w| {
        if w.get_active() {
            set_performance(1);
            w.set_sensitive(false);
        }
    });

    performance_battery_saver.connect_activate(|w| {
        if w.get_active() {
            set_performance(2);
            w.set_sensitive(false);
        }
    });

    performance_better.connect_activate(|w| {
        if w.get_active() {
            set_performance(3);
            w.set_sensitive(false);
        }
    });
    performance_best.connect_activate(|w| {
        if w.get_active() {
            set_performance(4);
            w.set_sensitive(false);
        }
    });

    // set inital
    {
        let text = get_perf_mode().unwrap_or_else(|_| format!("-1"));
        let normal = &text == "1";
        let battery = &text == "2";
        let better = &text == "3";
        let best = &text == "4";

        performance_normal.set_active(normal);
        performance_normal.set_sensitive(!normal);
        performance_battery_saver.set_active(battery);
        performance_battery_saver.set_sensitive(!battery);
        performance_better.set_active(better);
        performance_better.set_sensitive(!better);
        performance_best.set_active(best);
        performance_best.set_sensitive(!best);
    }

    {
        let mode = get_dgpu_mode();
        match mode {
            Ok(m) => {
                let enabled = if m == sys::dgpu::PowerState::On {
                    true
                } else {
                    false
                };
                set_dgpu(enabled);
            }
            Err(_) => dgpu.set_sensitive(false),
        }
    }

    {
        let can_detach = get_detach_available();
        if !can_detach {
            latch.set_sensitive(false);
        }
    }

    thread_performance().attach(None, move |text| {
        let normal = &text == "1";
        let battery = &text == "2";
        let better = &text == "3";
        let best = &text == "4";

        performance_normal.set_active(normal);
        performance_normal.set_sensitive(!normal);
        performance_battery_saver.set_active(battery);
        performance_battery_saver.set_sensitive(!battery);
        performance_better.set_active(better);
        performance_better.set_sensitive(!better);
        performance_best.set_active(best);
        performance_best.set_sensitive(!best);

        glib::Continue(true)
    });

    tray.set_status(AppIndicatorStatus::Active);
    tray.set_menu(&mut menu);
    menu.show_all();

    gtk::main();
}

fn set_dgpu(enabled: bool) {
    let state = if enabled { "off" } else { "on" };
    Command::new("sudo")
        .arg("surface")
        .arg("dgpu")
        .arg("set")
        .arg(state)
        .spawn()
        .expect("Failed to set dgpu");
}

fn set_performance(mode: u32) {
    Command::new("sudo")
        .arg("surface")
        .arg("performance")
        .arg("set")
        .arg(mode.to_string())
        .spawn()
        .expect("Failed to set performance mode");
}

fn thread_performance() -> Receiver<String> {
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        let (tx2, rx2) = channel();
        let mut watcher = watcher(tx2, Duration::from_secs(1)).unwrap();

        watcher
            .watch(
                "/sys/bus/surface_aggregator/devices/01:03:01:00:01/perf_mode",
                RecursiveMode::NonRecursive,
            )
            .unwrap();
        loop {
            match rx2.recv() {
                Ok(_) => match get_perf_mode() {
                    Ok(m) => tx.send(m).expect("Couldn't send data to channel"),
                    Err(_) => tx
                        .send(format!("-1"))
                        .expect("Couldn't send data to channel"),
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    rx
}

fn thread_dgpu() -> Receiver<String> {
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        let (tx2, rx2) = channel();
        let mut watcher = watcher(tx2, Duration::from_secs(1)).unwrap();

        watcher
            .watch(
                "/sys/bus/surface_aggregator/devices/01:03:01:00:01/perf_mode",
                RecursiveMode::NonRecursive,
            )
            .unwrap();
        loop {
            match rx2.recv() {
                Ok(_) => match get_perf_mode() {
                    Ok(m) => tx.send(m).expect("Couldn't send data to channel"),
                    Err(_) => tx
                        .send(format!("-1"))
                        .expect("Couldn't send data to channel"),
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    rx
}

fn get_perf_mode() -> Result<String> {
    let dev = sys::perf::Device::open()?;
    let mode = dev.get_mode()?;

    Ok(format!("{}", mode.short_str()))
}

fn get_dgpu_mode() -> Result<sys::dgpu::PowerState> {
    let dev = sys::dgpu::Device::open()?;
    let status = dev.get_power()?;
    Ok(status)
}

fn get_detach_available() -> bool {
    let dev = sys::latch::Device::open();
    match dev {
        Ok(device) => match device.get_opmode() {
            Ok(opmode) => return opmode == sys::latch::OpMode::Laptop,
            Err(_) => return false,
        },
        Err(_) => return false,
    }
}
