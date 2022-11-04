use crate::*;
use core::panic;

#[panic_handler]
fn panic(info: &panic::PanicInfo) -> ! {
    crit!("{}", info);
    loop {}
}
