#![no_std]
#![no_main]

use crate::{arch::io::Pio, io::Io};

mod arch;
mod io;
mod macros;
mod panic;

#[allow(unconditional_panic)]
pub fn kmain() {
    Pio::<u8>::new(0xe9).write(b'A');
    verbose!("GardenOS starting");
    debug!("GardenOS starting");
    info!("GardenOS starting");
    warn!("GardenOS starting");
    error!("GardenOS starting");
    panic!("here")
}
