use crate::start::_start;
use crate::{OS_STACK, OS_STACK_SIZE};
use core::arch::asm;

// ANCHOR: entry
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _entry() -> ! {
    asm!(
        "la sp, {STACK0}",
        "li t0, {OS_STACK_SIZE}",
        "add sp, sp, t0",
        "call {START}",
        STACK0 = sym OS_STACK,
        OS_STACK_SIZE = const OS_STACK_SIZE,
        START = sym _start,
        options(noreturn)
    )
}
// ANCHOR_END: entry
