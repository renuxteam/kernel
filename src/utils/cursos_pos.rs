use core::ptr::write_volatile;

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

pub fn set_cursor_position(x: usize, y: usize) {
    let offset = (y * VGA_WIDTH + x) * 2;
    unsafe {
        core::ptr::write_volatile(VGA_BUFFER.add(offset), b' '); // Attribute byte (white on black)
    }
}
