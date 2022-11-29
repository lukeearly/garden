#[cfg(any(feature = "debug_port_e9", feature = "debug_serial"))]
use super::io::Pio;
#[cfg(feature = "debug_port_e9")]
use crate::io::Io;

#[cfg(feature = "debug_vga")]
use super::dev::vga_text::{Attribute, Color, Cursor};

use core::fmt::{Result, Write};

pub struct Writer {
    #[cfg(feature = "debug_port_e9")]
    port_e9: Pio<u8>,
    #[cfg(feature = "debug_vga")]
    vga: Cursor,
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        #[cfg(feature = "debug_port_e9")]
        for &b in s.as_bytes() {
            self.port_e9.write(b);
        }

        #[cfg(feature = "debug_vga")]
        self.vga.write_str(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        #[cfg(feature = "debug_port_e9")]
        port_e9: Pio::<u8>::new(0xe9),
        #[cfg(feature = "debug_vga")]
        vga: Cursor::new(
            0,
            0,
            80,
            25,
            Attribute::new(Color::GRAY, Color::BLACK, false, false)
        )
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        write!(crate::arch::log::WRITER.lock(), $($arg)*).unwrap();
    });
}
