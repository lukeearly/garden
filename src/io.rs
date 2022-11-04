use core::{
    mem::MaybeUninit,
    ptr::{read_volatile, write_volatile},
};

pub trait Io {
    type Value: Copy;

    fn read(&self) -> Self::Value;

    fn write(&mut self, value: Self::Value);
}

#[repr(transparent)]
pub struct Mmio<T> {
    value: MaybeUninit<T>,
}

impl<T: Copy> Io for Mmio<T> {
    type Value = T;

    fn read(&self) -> Self::Value {
        unsafe { read_volatile(self.value.as_ptr()) }
    }

    fn write(&mut self, value: Self::Value) {
        unsafe {
            write_volatile(self.value.as_mut_ptr(), value);
        }
    }
}
