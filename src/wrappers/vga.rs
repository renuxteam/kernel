const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;

pub fn put_char_at(x: usize, y: usize, charater: u8, color: u8) {
    let offset = (y * VGA_WIDTH + x) * 2;
    unsafe {
        core::ptr::write_volatile(VGA_BUFFER.add(offset), charater);
        core::ptr::write_volatile(VGA_BUFFER.add(offset + 1), color);
    }
}

// Import vga.c in drivers/gpu/c/
unsafe extern "C" {
    fn vga_clear_screen();
    fn vga_print(text: *const u8);
}

pub fn clear_screen() {
    unsafe {
        vga_clear_screen();
    }
}

pub fn print_text(text: &str) {
    unsafe {
        vga_print(text.as_ptr());
    }
}
