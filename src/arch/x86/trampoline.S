.section .boot_stack, "aw", @nobits
.align 16
stack_bottom:
.skip 16384
stack_top:

.section .bss, "aw", @nobits
.align 4096
boot_page_directory:
.skip 4096
boot_page_table1:
.skip 4096

.section .multiboot.text, "ax"
.global _start
.type _start, @function
_start:
    # page table is in bss, and has to be offset by -3G to access before paging
    movl $(boot_page_table1 - _phys_virt_offset), %edi # pointer to page table entry
	movl $0, %esi # pointer to physical memory
	movl $1024, %ecx # counter to 1024 pages = 4M

.set PAGE_PRESENT 1<<0
.set PAGE_WRITABLE 1<<1

1:
 	movl %esi, %edx
    # enable present on all pages
	orl $(PAGE_PRESENT), %edx

    cmpl $_kernel_writable_start, %esi
    jl 3f

    # enable writeable on kernel data and bss only
	orl $(PAGE_WRITABLE), %edx
3:
	movl %edx, (%edi)

    # increment physical memory and pointer into page table
    addl $4096, %esi
    addl $4, %edi
    loop 1b

    # map the same physical region to both 0-4M (identity mapping) and 3G + 0-4M (permanent kernel position)
    movl $(boot_page_table1 - _phys_virt_offset + (PAGE_PRESENT | PAGE_WRITABLE)), boot_page_directory - _phys_virt_offset + 0
	movl $(boot_page_table1 - _phys_virt_offset + (PAGE_PRESENT | PAGE_WRITABLE)), boot_page_directory - _phys_virt_offset + _phys_virt_offset / 4096 / 1024

    # place the page directory address in cr3
	movl $(boot_page_directory - _phys_virt_offset), %ecx
	movl %ecx, %cr3

.set CR0_PG 1<<31
.set CR0_WP 1<<16

    # enable paging
	movl %cr0, %ecx
	orl $(CR0_PG | CR0_WP), %ecx
	movl %ecx, %cr0

    # jump to absolute position in higher half
    lea 4f, %ecx
	jmp *%ecx

.text

4:
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
