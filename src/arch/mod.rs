#[cfg(target_arch = "x86_64")]
#[macro_use]
pub mod x86_64;

// TODO eliminate pub use *, replace with unified interfaces like HostArch
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

pub trait Architecture<HostSize> {
    const PAGE_SIZE: HostSize;
}

#[cfg(target_arch = "x86_64")]
pub type HostArch = X86_64;
