#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod print;
use print::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Hello,\nWorld!".as_ref(), 0xf, (0,0));
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print(info.payload().downcast_ref::<&str>().unwrap().as_ref(), 0x4, (0,0));
    loop {}
}