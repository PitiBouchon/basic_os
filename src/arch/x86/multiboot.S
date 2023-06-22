.pushsection .multiboot_header

.set ALIGN,    1<<0
.set MEMINFO,  1<<1
.set FLAGS,    ALIGN | MEMINFO
.set MAGIC,    0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)

.align 4

.long   MAGIC
.long   FLAGS
.long   CHECKSUM

.popsection
