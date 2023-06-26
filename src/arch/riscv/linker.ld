OUTPUT_ARCH("riscv");
ENTRY(_entry); /* Must be the same as in entry.rs */

SECTIONS
{
    . = 0x80200000; /* TODO : remove this ? I think this is not necessary */

    PROVIDE(_kernel_start = .);

    .text : {
        *(.text.entry)
        ASSERT(. - _kernel_start > 0, "Error: no entry code");
        *(.text .text.*)
    }

    .rodata : {
        . = ALIGN(16);
        *(.srodata .srodata.*) /* do not need to distinguish this from .rodata */
        . = ALIGN(16);
        *(.rodata .rodata.*)
    }

    .data : {
        . = ALIGN(16);
        PROVIDE( __global_pointer$ = . ); /* Should be placed at the middle of .sdata see : https://gnu-mcu-eclipse.github.io/arch/riscv/programmer/ */
        *(.sdata .sdata.*) /* do not need to distinguish this from .data */
        . = ALIGN(16);
        *(.data .data.*)
    }

    .bss : {
        PROVIDE(_start_bss = .);
        . = ALIGN(16);
        *(.sbss .sbss.*) /* do not need to distinguish this from .bss */
        . = ALIGN(16);
        *(.bss .bss.*)
        PROVIDE(_end_bss = .);
    }

    PROVIDE(_kernel_end = .);
};
