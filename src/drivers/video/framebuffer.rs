#[derive(Debug, Copy, Clone)]
pub struct FramebufferInfo {
    pub address: usize,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bpp: u8,
}

pub struct Framebuffer {
    pub info: FramebufferInfo,
}

impl Framebuffer {
    pub fn new(info: FramebufferInfo) -> Self {
        Self { info }
    }

    fn framebuffer_ptr(&self) -> *mut u8 {
        self.info.address as *mut u8
    }

    pub fn clear(&self, color: [u8; 4]) {
        let size = self.info.pitch * self.info.height;
        let ptr = self.framebuffer_ptr();
        unsafe {
            for i in (0..size).step_by(4) {
                core::ptr::write_volatile(ptr.add(i), color[0]);
                core::ptr::write_volatile(ptr.add(i + 1), color[1]);
                core::ptr::write_volatile(ptr.add(i + 2), color[2]);
                core::ptr::write_volatile(ptr.add(i + 3), color[3]);
            }
        }
    }

    pub fn put_pixel(&self, x: usize, y: usize, color: [u8; 4]) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }
        let offset = (y * self.info.pitch + x * 4) as usize;
        let ptr = unsafe { self.framebuffer_ptr().add(offset) };
        unsafe {
            for i in 0..4 {
                core::ptr::write_volatile(ptr.add(i), color[i]);
            }
        }
    }
}
