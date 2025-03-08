use crate::wrappers::vga::{clear_screen, print_text, initalize_driver};

pub fn start()
{
  initalize_driver(); // Initialize the VGA driver
  print_text("Welcome to Renux OS"); // Write a debug message

  // write another debug mensage
}
