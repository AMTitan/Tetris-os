const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub fn print(stirng:&[u8], color:u8, pos:(usize, usize)) {
    let vga_buffer = 0xb8000 as *mut u8;
    let mut offset = (pos.1*BUFFER_WIDTH)+pos.0;

    for (i, &byte) in stirng.iter().enumerate() {
        unsafe {
            *vga_buffer.offset((i + offset) as isize * 2) = byte;
            *vga_buffer.offset((i + offset) as isize * 2 + 1) = color;
        }
    }
}

pub fn clear() {
    let vga_buffer = 0xb8000 as *mut u8;
    for i in 0..4000 {
        unsafe {
            *vga_buffer.offset(i as isize) = 0x0;
        }
    }
}