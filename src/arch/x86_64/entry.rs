use super::interrupt::{IDT, IDT_DESCRIPTOR};

#[no_mangle]
pub extern "C" fn kentry() -> ! {
    unsafe { IDT_DESCRIPTOR.load() };
    crate::kmain();
    loop {}
}
