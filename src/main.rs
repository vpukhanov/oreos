#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oreos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use oreos::println;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("OreOS v{}", VERSION);
    println!("Hello from _start()!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oreos::test_panic_handler(info)
}
