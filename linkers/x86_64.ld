ENTRY(_start)

SECTIONS
{
    _phys_virt_offset = 3072M;

    . = 1M;

    _kernel_phys_start = .;
    _kernel_virt_start = . + _phys_virt_offset;
    .multiboot.data : {
        *(.multiboot.data)
    }
    _kernel_nonwritable_start = .;
    .multiboot.text : {
        *(.multiboot.text)
    }

    . += _phys_virt_offset;
    .text ALIGN(4K) : AT (ADDR (.text) - _phys_virt_offset) {
        *(.text)
    }
    .rodata ALIGN(4K) : AT (ADDR (.rodata) - _phys_virt_offset) {
        *(.rodata)
        *(.gdt)
    }

    _kernel_writable_start = . - _phys_virt_offset;

    .data ALIGN(4K) : AT (ADDR (.data) - _phys_virt_offset) {
        *(.data)
    }
    .bss ALIGN(4K) : AT (ADDR (.bss) - _phys_virt_offset) {
        *(COMMON)
        *(.bss)
        *(.boot_stack)
    }

    _kernel_virt_end = .;
    _kernel_phys_end = . - _phys_virt_offset;
}
