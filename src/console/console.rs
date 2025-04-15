use crate::init_system::video::VideoDriver;

pub struct Console {
    pub x: usize,
    pub y: usize,
}

impl Terminal {
    pub fn new() -> Self {
        Console { x: 0, y: 0 }
    }

    pub fn write_str(&mut self, text: str) {
        if c == '\n' {
            self.x = 0;
            self.y += 1;
            continue;
        } 
        unsafe {
            if let Some(driver) = &mut VIDEO {
                driver.draw_char(self.x, self.y, c);
            }
        }
    }

    pub fn clear_screen(&mut self) {
        unsafe {
            if let Some(driver) = &mut VIDEO {
                driver.clear_screen();
            }
        }
        self.x = 0;
        self.y = 0;
    }
}