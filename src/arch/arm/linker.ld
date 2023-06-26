ENTRY(_entry); /* Must be the same as in entry.rs */

SECTIONS
{
    . = 0x40000000;

    PROVIDE(_kernel_start = .);

    .text : {
        *(.text.entry)
        ASSERT(. - _kernel_start > 0, "No Entry Code !");
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
