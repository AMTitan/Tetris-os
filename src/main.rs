#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod print;
use print::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Hello, World!".as_ref(), 0xf, (10,5));
    clear();
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}