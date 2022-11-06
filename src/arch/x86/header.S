/* multiboot header constants */
.set MAGIC, 0x1BADB002
.set ALIGN, 1<<0
.set MEMMAP, 1<<1
.set VIDEO, 1<<2
.set VMODE, 1 // text
.set VWIDTH, 25
.set VHEIGHT, 80
.set VDEPTH, 0 // only for linear graphics mode
.set FLAGS, ALIGN | MEMMAP | VIDEO
.set CHECK, -(MAGIC + FLAGS)

.section .multiboot.header, "a"
.align 4
.long MAGIC
.long FLAGS
.long CHECK
.skip 20
.long VMODE
.long VWIDTH
.long VHEIGHT
.long VDEPTH
