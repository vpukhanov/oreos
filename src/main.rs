#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("OreOS v{}", VERSION);
    println!("Hello from _start()!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
