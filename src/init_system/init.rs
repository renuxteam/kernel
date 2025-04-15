use crate::wrappers::vga::{clear_screen, print_text};
use crate::utils::delay::delay;
use crate::wrappers::vga::put_char_at;
use crate::console::boot_anim;



pub fn start() {
  boot_anim::loading_animation();
}
