use core::sync::atomic::{AtomicBool, Ordering};
use crate::wrappers::vga::{*};
use crate::utils::delay::delay;


const SPINNER: &[u8] = b"\\|/-";

static ALREADY_LOADED: AtomicBool = AtomicBool::new(false);

pub fn loading_animation() {
    unsafe {
        if ALREADY_LOADED.swap(true, Ordering::SeqCst) {
            return;
        }
    }
    clear_screen();
  
    let msg = b"Initializing RenuxOS Kernel";
    for (i, &byte) in msg.iter().enumerate() {
      put_char_at(i, 0, byte, 0x0F);
    }
  
    let spinner_x = msg.len() + 2;
    let spinner_y = 0;
  
  
    for (i) in 0..32 {
      let frame = SPINNER[i % SPINNER.len()];
      put_char_at(spinner_x, spinner_y,frame, 0x0E);
      delay(1000);
    }
    put_char_at(spinner_x, spinner_y, b' ', 0x0F); // Clear the spinner
    put_char_at(msg.len() + 2, 0, b'O', 0x0A); // verde
    put_char_at(msg.len() + 3, 0, b'K', 0x0A);
}