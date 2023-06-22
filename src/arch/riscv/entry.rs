use crate::arch::riscv::machine::set_fdt;
use crate::{OS_STACK, OS_STACK_SIZE};
use core::arch::asm;
use fdt::Fdt;
use sbi_print::println;

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
        START = sym start,
        options(noreturn)
    )
}
// ANCHOR_END: entry

#[no_mangle] // This function must have the same name as in entry.S
pub unsafe extern "C" fn start(hart_id: usize, dtb: usize) -> ! {
    extern "C" {
        static mut _start_bss: u8;
        static mut _end_bss: u8;
    }

    // Zeroing the .BSS section (which correspond to uninitialized values)
    // TODO : I think it should be done in assembly
    let bss_size = &_end_bss as *const u8 as usize - &_start_bss as *const u8 as usize;
    core::ptr::write_bytes(&mut _start_bss as *mut u8, 0, bss_size);

    set_fdt(dtb).unwrap();

    crate::main(hart_id)
}
