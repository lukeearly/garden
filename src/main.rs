#![no_std]
#![no_main]

mod arch;
mod io;
mod macros;
mod panic;

#[allow(unconditional_panic)]
pub fn kmain() {
    verbose!("GardenOS starting");
    debug!("GardenOS starting");
    info!("GardenOS starting");
    warn!("GardenOS starting");
    error!("GardenOS starting");
    panic!("here")
}
