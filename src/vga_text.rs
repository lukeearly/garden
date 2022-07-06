use core::{ptr,fmt, panic};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    BLACK = 0,
    BLUE,
    GREEN,
    CYAN,
    RED,
    MAGENTA,
    BROWN,
    GRAY
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Attribute(u8);

impl Attribute {
    pub fn new(fg: Color, bg: Color, bright: bool, blink: bool) -> Self {
        Attribute((fg as u8) | (bright as u8) << 3 | (bg as u8) << 4 | (blink as u8) << 7)
    }

    pub fn fg(&self) -> u8 {
        (self.0 & 7)// as Color
    }

    pub fn bg(&self) -> u8 {
        (self.0 >> 4 & 7)// as Color
    }

    pub fn bright(&self) -> bool {
        (self.0 & 8) != 0
    }

    pub fn blink(&self) -> bool {
        (self.0 & 128) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
struct VGAChar {
    char: u8,
    attr: Attribute
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
pub struct TextBuffer {
    chars: [[VGAChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

use lazy_static::lazy_static;
use spin::Mutex;

const VGA_ADDRESS: usize = 0xb8000;
lazy_static! {
    pub static ref MAIN_BUFFER: Mutex<&'static mut TextBuffer> = Mutex::new(unsafe { &mut *(VGA_ADDRESS as *mut TextBuffer) });
}

pub struct Cursor {
    row: usize,
    col: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    current_attr: Attribute,
    buffer: &'static Mutex<&'static mut TextBuffer>
}

impl Cursor {
    pub fn new(x: usize, y: usize, w: usize, h: usize, attr: Attribute) -> Self {
        Cursor { row: y, col: x, x, y, w, h, current_attr: attr, buffer: &MAIN_BUFFER }
    }

    pub fn set_attr(&mut self, attr: Attribute) {
        self.current_attr = attr;
    }

    // compiler cannot elide or reorder volatile writes and reads
    fn write_pos(&mut self, r: usize, c: usize, vc: VGAChar) {
        unsafe { ptr::write_volatile(self.buffer.lock().chars[r].as_mut_ptr().offset((c) as isize), vc) }
    }

    fn read_pos(&self, r: usize, c: usize) -> VGAChar {
        unsafe { ptr::read_volatile(self.buffer.lock().chars[r].as_ptr().offset((c) as isize)) }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if self.col < self.x {
            self.col = self.x;
        }
        if self.row < self.y {
            self.row = self.y;
        }

        if byte == '\n' as u8 {
            self.new_line();
            return;
        } if self.col >= self.x + self.w || self.row >= self.y + self.h {
            self.new_line();
        }

        let vc = VGAChar { attr: self.current_attr, char: byte };
        self.write_pos(self.row, self.col, vc);

        self.col += 1;
    }

    fn new_line(&mut self) {
        self.col = self.x;
        self.row += 1;
        if self.row >= self.y + self.h {
            let magnitude = self.y + self.h - self.row + 1;
            for r in magnitude..self.y + self.h {
                for c in self.y..self.y + self.h {
                    self.write_pos(r - magnitude, c, self.read_pos(r, c));
                }
            }

            let vc = VGAChar { attr: Attribute::new(Color::GRAY, Color::BLACK, false, false), char: ' ' as u8 };
            for c in self.x..self.x + self.w {
                self.write_pos(self.y + self.h - 1, c, vc);
            }
            self.row = self.y + self.h - 1;
        }
    }
}

impl fmt::Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(
                if b < 0x7e {
                    b
                } else {
                    0xfe
                }
            );
        }
        Ok(())
    }
}