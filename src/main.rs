#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Hello, World!".as_ref(), 0xf);
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn print(stirng:&[u8], color:u8) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in stirng.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = color;
        }
    }
}