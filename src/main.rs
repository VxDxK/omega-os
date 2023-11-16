#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::omega_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod omega_test;
mod serial;
mod vga_buffer;
mod interrupts;
mod gdt;

use core::panic::PanicInfo;
use bootloader::entry_point;

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World format println {}", 228);

    gdt::init_gdt();
    interrupts::init_idt();

    x86_64::instructions::interrupts::int3();

    loop {}
}

// entry_point!(fr);