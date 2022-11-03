use core::panic;
use crate::*;

#[panic_handler]
fn panic(info: &panic::PanicInfo) -> ! {
    crit!("{}", info);
    loop {}
}