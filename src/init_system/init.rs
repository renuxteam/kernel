use crate::{
    drivers::video::framebuffer::{self, FramebufferInfo},
    utils::delay,
};

pub fn init_start() {
    let fb_info = FramebufferInfo {
        address: 0xF0000000, // Replace with actual framebuffer address
        width: 800,
        height: 600,
        pitch: 320,
        bpp: 32,
    };

    let fb = framebuffer::Framebuffer::new(fb_info);

    fb.clear([0, 0, 0, 255]); // Clear the screen to black
    fb.put_pixel(10, 10, [255, 0, 0, 255]); // Draw a red pixel at (100, 100)
}
