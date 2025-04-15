use crate::console::boot_anim;
use crate::utils::delay::delay;
use crate::wrappers::vga::put_char_at;
use crate::wrappers::vga::{clear_screen, print_text};

pub fn start() {
    boot_anim::loading_animation();
}
