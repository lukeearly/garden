#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::kmain();
    loop { };
}