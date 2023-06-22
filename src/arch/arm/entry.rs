use crate::{OS_STACK, OS_STACK_SIZE};
use core::arch::asm;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _entry() -> ! {
    asm!(
        // TODO : setup the stack pointer and clear the bss
        "b {START}",
        // STACK0 = sym OS_STACK,
        // OS_STACK_SIZE = const OS_STACK_SIZE,
        START = sym start,
        options(noreturn)
    )
}

#[no_mangle]
pub unsafe extern "C" fn start() -> ! {
    // Just loop
    loop {}
}
