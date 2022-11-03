use crate::io::Io;
use core::arch::asm;
use core::marker::PhantomData;

#[repr(transparent)]
pub struct Pio<V> {
    port: u16,
    value: PhantomData<V>
}

impl<V> Pio<V> {
    pub fn new(port: u16) -> Self {
        Pio::<V> {
            port,
            value: PhantomData
        }
    }
}

impl Io for Pio<u8> {
    type Value = u8;

    #[inline(always)]
    fn read(&self) -> u8 {
        let mut value: u8;
        unsafe {
            asm!("in al, dx", out("al") value, in("dx") self.port, options(nostack));
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u8) {
        unsafe {
            asm!("out dx, al", in("al") value, in("dx") self.port, options(nostack));
        }
    }
}

impl Io for Pio<u16> {
    type Value = u16;

    #[inline(always)]
    fn read(&self) -> u16 {
        let mut value: u16;
        unsafe {
            asm!("in ax, dx", out("ax") value, in("dx") self.port, options(nostack));
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u16) {
        unsafe {
            asm!("out dx, ax", in("ax") value, in("dx") self.port, options(nostack));
        }
    }
}

impl Io for Pio<u32> {
    type Value = u32;

    #[inline(always)]
    fn read(&self) -> u32 {
        let mut value: u32;
        unsafe {
            asm!("in eax, dx", out("eax") value, in("dx") self.port, options(nostack));
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u32) {
        unsafe {
            asm!("out dx, eax", in("eax") value, in("dx") self.port, options(nostack));
        }
    }
}