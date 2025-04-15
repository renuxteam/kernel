#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Load Init system
mod init_system;
// Load utils
mod utils;
// Load multiboot
mod multiboot;
// Load console
mod console;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    wrappers::vga::clear_screen();
    wrappers::vga::print_text("Kernel Panic");
    loop {}
}

// Import wrappers
mod wrappers;
#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    init_system::init::start();
    #[cfg(test)]
    test_main();
    loop {}
}

fn trivial_assertion() {
    wrappers::vga::clear_screen();
    wrappers::vga::print_text("Trivial assertion");
    assert_eq!(1, 1);
    wrappers::vga::print_text("[OK]");
    loop {}
}
