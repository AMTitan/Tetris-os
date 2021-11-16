const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub fn print(stirng:&[u8], color:u8, pos:(usize, usize)) {
    let vga_buffer = 0xb8000 as *mut u8;
    let mut offset = (pos.1*BUFFER_WIDTH)+pos.0;

    for (i, &byte) in stirng.iter().enumerate() {
        if byte == 10 { // if is \n
            offset +=BUFFER_WIDTH-((offset + i)%BUFFER_WIDTH)-1;
        }
        else {
            unsafe {
                *vga_buffer.offset((i + offset) as isize * 2) = byte;
                *vga_buffer.offset((i + offset) as isize * 2 + 1) = color;
            }
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

pub fn get_size() -> (usize, usize) {
    return (BUFFER_WIDTH, BUFFER_HEIGHT);
}