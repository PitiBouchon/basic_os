OUTPUT_ARCH("i386:x86-64");
ENTRY(_entry); /* Must be the same as in entry.rs */

SECTIONS
{
    . = 0x00100000;

    PROVIDE(_kernel_start = .);

    .text : {
        PROVIDE(_check_multiboot = .);
        *(.multiboot_header)
        ASSERT(. - _check_multiboot > 0, "error: no multiboot code");
        *(.text.entry)
        *(.text .text.*)
    }

    .rodata : {
        *(.srodata .srodata.*)
        *(.rodata .rodata.*)
    }

    .data : {
        *(.sdata .sdata.*)
        *(.data .data.*)
    }

    .bss : {
        *(.sbss .sbss.*)
        *(.bss .bss.*)
    }

    PROVIDE(_kernel_end = .);
};
