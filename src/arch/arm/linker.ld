ENTRY(_entry); /* Must be the same as in entry.rs */

SECTIONS
{
    . = 0x40000000; /* TODO: Why this value ? */

    PROVIDE(_kernel_start = .);

    .text : {
        *(.text.entry)
        ASSERT(. - _kernel_start > 0, "No Entry Code !");
        *(.text .text.*)
    }

    .rodata : {
        . = ALIGN(16);
        *(.srodata .srodata.*)
        . = ALIGN(16);
        *(.rodata .rodata.*)
    }

    .data : {
        . = ALIGN(16);
        *(.sdata .sdata.*)
        . = ALIGN(16);
        *(.data .data.*)
    }

    .bss : {
        PROVIDE(_start_bss = .);
        . = ALIGN(16);
        *(.sbss .sbss.*)
        . = ALIGN(16);
        *(.bss .bss.*)
        PROVIDE(_end_bss = .);
    }

    PROVIDE(_kernel_end = .);
};
