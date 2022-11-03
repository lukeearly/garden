#![no_std]
#![no_main]

use core::{panic, fmt::Write};

mod vga_text;
use vga_text::{Attribute,Color,Cursor};

#[allow(unconditional_panic)]
pub fn kmain() {
    let mut main_cursor = Cursor::new(
        0, 0,
        80, 25,
        Attribute::new(Color::GREEN, Color::BLACK, false, false)
    );
    main_cursor.set_attr(Attribute::new(Color::RED, Color::BROWN, false, true));
    for i in 0..30 {
        write!(main_cursor, "{}. Hello, {}!\n", i, "world").unwrap();
    }

    let mut cursor = Cursor::new(
        40, 15,
        40, 4,
        Attribute::new(Color::RED, Color::BLACK, false, false)
    );
    write!(cursor, "Hello, {}!\n", "world").unwrap();
    
    panic!("here");
}

#[panic_handler]
fn panic(info: &panic::PanicInfo) -> ! {
    let mut vga_cursor = Cursor::new(
        40, 0,
        40, 25,
        Attribute::new(Color::GRAY, Color::RED, false, false)
    );
    write!(vga_cursor, "{}", info);

    loop {}
}