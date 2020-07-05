#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oreos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use oreos::println;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use oreos::memory;
    use oreos::memory::BootInfoFrameAllocator;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    println!("OreOS v{}", VERSION);
    println!("Hello from _start()!");

    oreos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    oreos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    oreos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oreos::test_panic_handler(info);
}
