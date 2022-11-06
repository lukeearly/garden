use core::arch::global_asm;

global_asm!(include_str!("./header.S"), options(att_syntax));
global_asm!(include_str!("./trampoline.S"), options(att_syntax));
