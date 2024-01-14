#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::omega_test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(const_mut_refs)]

extern crate alloc;

mod omega_test;
mod serial;
mod vga_buffer;
mod interrupts;
mod gdt;
mod memory;
mod allocator;
mod util;
mod task;

use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::{structures::paging::Page, VirtAddr};
use crate::memory::BootInfoFrameAllocator;
use crate::task::simple_executor::SimpleExecutor;
use crate::task::Task;
use crate::vga_buffer::TTY;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn kmain(boot_info: &'static BootInfo) -> ! {
    gdt::init_gdt();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Can`t initialize heap");



    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(task()));
    executor.run();

    loop { x86_64::instructions::hlt(); }
}

async fn async_number() -> u32 {
    52
}

async fn task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[cfg(not(test))]
entry_point!(kmain);