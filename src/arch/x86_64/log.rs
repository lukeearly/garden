use crate::io::Io;

use super::{
    dev::vga_text::{Attribute, Color, Cursor},
    io::Pio,
};
use core::fmt::{Result, Write};

pub struct Writer {
    qemu: Pio<u8>,
    vga: Cursor,
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        for &b in s.as_bytes() {
            self.qemu.write(b);
        }
        self.vga.write_str(s)
    }
}

use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        qemu: Pio::<u8>::new(0xe9),
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
