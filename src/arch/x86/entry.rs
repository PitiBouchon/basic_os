use crate::{OS_STACK, OS_STACK_SIZE};
use core::arch::asm;

core::arch::global_asm!(include_str!("multiboot.S"));

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _entry() -> ! {
    asm!(
        // TODO : do a boot things to pass from 32 bits to 64 bits (and this is annoying)
        "movw  $(2<<3), %ax",
        "movw    %ax, %ds",
        "movw    %ax, %es",
        "movw    %ax, %ss",
        "movw    $0, %ax",
        "movw    %ax, %fs",
        "movw    %ax, %gs",
        "mov $({STACK0} + {OS_STACK_SIZE}), %rsp",
        "mov {START}, %rax",
        "jmp *%rax",
        STACK0 = sym OS_STACK,
        OS_STACK_SIZE = const OS_STACK_SIZE,
        START = sym start,
        options(noreturn),
        options(att_syntax)
    )
}

#[no_mangle]
pub unsafe extern "C" fn start() -> ! {
    // Just loop
    loop {}
}
