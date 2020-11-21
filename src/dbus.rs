use super::sys::{dgpu, latch, perf};
use dbus::blocking::Connection;
use dbus::MethodErr;
use dbus_crossroads::Crossroads;
use std::error::Error;

pub fn start() -> Result<(), Box<dyn Error>> {
    let connection: Connection = Connection::new_session()?;
    connection.request_name("io.ebudd.surface-control-tray", false, true, false)?;
    let mut cr: Crossroads = Crossroads::new();

    let iface_token = cr.register("io.ebudd.surface-control-tray", |b| {
        b.method(
            "performance_set",
            ("mode",),
            ("code",),
            |_, _, (mode,): (u32,)| {
                let dev = match perf::Device::open() {
                    Ok(device) => device,
                    Err(_) => return Err(MethodErr::failed("Device not found")),
                };

                let mode_type = match perf::Mode::from_int(mode) {
                    Some(m) => m,
                    None => return Err(MethodErr::invalid_arg(&mode)),
                };

                match dev.set_mode(mode_type) {
                    Ok(_) => Ok((0,)),
                    Err(_) => Err(MethodErr::failed("Failed to set mode")),
                }
            },
        );

        b.method(
            "dgpu_set",
            ("status",),
            ("code",),
            |_, _, (status,): (bool,)| {
                let dev = match dgpu::Device::open() {
                    Ok(device) => device,
                    Err(_) => return Err(MethodErr::failed("Device not found")),
                };

                let power_state = dgpu::PowerState::from_bool(status);

                match dev.set_power(power_state) {
                    Ok(_) => Ok((0,)),
                    Err(_) => Err(MethodErr::failed("Failed to set power")),
                }
            },
        );

        b.method("latch_unlock", (), ("code",), |_, _, ()| {
            let dev = match latch::Device::open() {
                Ok(device) => device,
                Err(_) => return Err(MethodErr::failed("Device not found")),
            };

            match dev.latch_unlock() {
                Ok(_) => Ok((0,)),
                Err(_) => Err(MethodErr::failed("Failed to unlock")),
            }
        });

        b.method("dgpu_get", (), ("mode",), |_, _, ()| {
            let dev = match dgpu::Device::open() {
                Ok(device) => device,
                Err(_) => return Err(MethodErr::failed("Device not found")),
            };

            match dev.get_power() {
                Ok(p) => Ok((p.as_bool(),)),
                Err(_) => Err(MethodErr::failed("Failed to set mode")),
            }
        });
    });

    // Let's add the "/hello" path, which implements the com.example.dbustest interface,
    // to the crossroads instance.
    cr.insert("/", &[iface_token], {});

    // Serve clients forever.
    cr.serve(&connection)?;
    unreachable!()
}
