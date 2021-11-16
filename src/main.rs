#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod print;
mod board;
mod string_format;
use print::*;
use string_format::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Hello,\nWorld!".as_ref(), 0xf, (0,0));
    board::board_template();
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print(info.payload().downcast_ref::<&str>().unwrap().as_ref(), 0x4, (0,0));
    loop {}
}