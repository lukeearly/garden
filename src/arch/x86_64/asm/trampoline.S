.section .boot_stack, "aw", @nobits
.align 16
stack_bottom:
.skip 16384
stack_top:

.section .bss, "aw", @nobits
.align 4096
boot_page_map:
.skip 4096
boot_page_directory_pointer_table1:
.skip 4096
boot_page_directory1:
.skip 4096
boot_page_table1:
.skip 4096
boot_page_table2:
.skip 4096

.set _phys_virt_offset, 0xC0000000

.section .multiboot.text, "ax"
.global _start
.type _start, @function
.code32
_start:
    # page table is in bss, and has to be offset by -3G to access before paging
    movl $(boot_page_table1 - _phys_virt_offset), %edi # pointer to page table entry
	movl $0, %esi # pointer to physical memory
	movl $1024, %ecx # counter to 2 * 512 pages = 4M

.set PAGE_PRESENT, 1<<0
.set PAGE_WRITABLE, 1<<1

1:
 	movl %esi, %edx
    # enable present on all pages
	orl $(PAGE_PRESENT), %edx

    cmpl $_kernel_writable_start, %esi
    jl 3f

    # enable writable on kernel data and bss only
	orl $(PAGE_WRITABLE), %edx
3:
	movl %edx, (%edi)

    # increment physical memory and pointer into page table
    addl $4096, %esi
    addl $8, %edi
    loop 1b

    # identity map the two page tables into the first two entries of the page directory:
    movl $(boot_page_table1 - _phys_virt_offset + (PAGE_PRESENT | PAGE_WRITABLE)), boot_page_directory1 - _phys_virt_offset + 0
    movl $(boot_page_table2 - _phys_virt_offset + (PAGE_PRESENT | PAGE_WRITABLE)), boot_page_directory1 - _phys_virt_offset + 8
    
    # map the same physical region to both 0-4M (identity mapping) and 3G + 0-4M (permanent kernel position)
    movl $(boot_page_directory1 - _phys_virt_offset + (PAGE_PRESENT | PAGE_WRITABLE)), boot_page_directory_pointer_table1 - _phys_virt_offset + 0
    movl $(boot_page_directory1 - _phys_virt_offset + (PAGE_PRESENT | PAGE_WRITABLE)), boot_page_directory_pointer_table1 - _phys_virt_offset + 24

    movl $(boot_page_directory_pointer_table1 - _phys_virt_offset + (PAGE_PRESENT | PAGE_WRITABLE)), boot_page_map - _phys_virt_offset + 0

.set CR4_PAE, 1<<5
    # enable PAE
	movl %cr4, %ecx
	orl $(CR4_PAE), %ecx
	movl %ecx, %cr4

    # place the level 4 page map address in cr3
	movl $(boot_page_map - _phys_virt_offset), %ecx
	movl %ecx, %cr3

.set MSR_EFER, 0xC0000080
.set EFER_LM, 1<<8
    movl $(MSR_EFER), %ecx
    rdmsr
    orl $(EFER_LM), %eax
    wrmsr

.set CR0_PG, 1<<31
.set CR0_WP, 1<<16

    xchgw %bx, %bx

    # enable paging
	movl %cr0, %ecx
	orl $(CR0_PG | CR0_WP), %ecx
	movl %ecx, %cr0

    lgdt gdt_pointer

    // ljmp   $0x08, 4f
    // jmp $(gdt_code - gdt_null), *%ecx

4:
//     cli
//     movw $(gdt_data - gdt_null), %ax
//     movw %ax, %ds
//     movw %ax, %es
//     movw %ax, %fs
//     movw %ax, %gs
//     movw %ax, %ss

    # jmp to absolute position in higher half
    lea 5f, %ecx
	jmp *%ecx

.section .text, "ax"
5:
	# flush TLB
	movl %cr3, %ecx
	movl %ecx, %cr3

    movl $stack_top, %esp
    movl $stack_top, %ebp

    call kentry

    # clear interrupts and loop forever
    cli
loop:
    hlt
    jmp loop

.set GDT_PRESENT,   1 << 7
.set GDT_NOTSYS,    1 << 4
.set GDT_EXEC,      1 << 3
.set GDT_DC,        1 << 2
.set GDT_RW,        1 << 1
.set GDT_ACCESSED,  1 << 0

.set GDT_GRAN_4K,   1 << 7
.set GDT_SZ_32,     1 << 6
.set GDT_LONG_MODE, 1 << 5

.section .gdt, "a"
gdt_null:
.quad 0
gdt_code:
.long 0x0000ffff                                        # limit (0-15) 0xffff, base (0-15) 0x0000
.byte 0                                                 # base (16-23) 0x00
.byte GDT_PRESENT | GDT_NOTSYS | GDT_EXEC | GDT_RW      # access
.byte GDT_GRAN_4K | GDT_LONG_MODE | 0xf                 # limit (16-19) 0xf, flags
.byte 0                                                 # base (24-31) 0x00
// .long 0x0000FFFF
// .long 0x00AF9A00
gdt_data:
.long 0x0000ffff                                        # limit (0-15) 0xffff, base (0-15) 0x0000
.byte 0                                                 # base (16-23) 0x00
.byte GDT_PRESENT | GDT_NOTSYS | GDT_RW                 # access
.byte GDT_GRAN_4K | GDT_SZ_32 | 0xf                     # limit (16-19) 0xf, flags
.byte 0                                                 # base (24-31) 0x00
// .long 0x0000FFFF
// .long 0x00CF9200
gdt_tss:
// .long 0x00000068
// .long 0x00CF8900
.align 4
.word 0
gdt_pointer:
.word gdt_pointer - gdt_null
.long gdt_null