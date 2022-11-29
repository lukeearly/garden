use super::Architecture;

pub mod asm;
pub mod dev;
pub mod entry;
pub mod interrupt;
pub mod io;
#[macro_use]
pub mod log;

pub struct X86_64;

impl Architecture<usize> for X86_64 {
    const PAGE_SIZE: usize = 4096;
}
