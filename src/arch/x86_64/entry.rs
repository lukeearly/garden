use core::arch::asm;

use crate::io::Io;
use super::io::Pio;

#[no_mangle]
pub extern "C" fn kentry() -> ! {
    Pio::<u8>::new(0xe9).write(b'G');
    unsafe { 
        asm!("
            mov edi, 0xB8000
            mov rcx, 500
            mov rax, 0x1F201F201F201F20
            rep stosq
        ");
    }
    crate::kmain();
    loop {}
}
