#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sbi_print::{print, println};

mod arch;
pub mod start;

const OS_STACK_SIZE: usize = 65536;

#[repr(C, align(16))]
struct Stack([u8; OS_STACK_SIZE]);

#[no_mangle]
static OS_STACK: Stack = Stack([0; OS_STACK_SIZE]);

fn main(hart_id: usize, dtb: usize) -> ! {
    println!("Hello from HartId : {} | Dtb at addr: 0x{:x}", hart_id, dtb);

    if cfg!(test) {
        println!("TEST MODE");
    } else {
        println!("NORMAL MODE");
    }

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC]: {:?}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
