#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(slice_as_chunks)]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(target_arch = "riscv64")]
use sbi_print::println;

mod arch;
mod interface;

const OS_STACK_SIZE: usize = 65536;

#[repr(C, align(16))]
struct Stack([u8; OS_STACK_SIZE]);

#[no_mangle]
static OS_STACK: Stack = Stack([0; OS_STACK_SIZE]);

fn main(hart_id: usize) -> ! {
    #[cfg(target_arch = "riscv64")]
    println!("Hello from HartId : {}", hart_id);

    // if cfg!(test) {
    //     println!("TEST MODE");
    // } else {
    //     println!("NORMAL MODE");
    // }

    // #[cfg(test)]
    // test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(target_arch = "riscv64")]
    println!("[PANIC]: {:?}", info);
    loop {}
}

// #[cfg(test)]
// fn test_runner(tests: &[&dyn Fn()]) {
//     println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//     }
// }
//
// #[test_case]
// fn trivial_assertion() {
//     println!("trivial assertion... ");
//     assert_eq!(1, 1);
//     println!("[ok]");
// }
