pub fn delay(ms: u64) {
    for _ in 0..1_920_000 {
        unsafe { core::arch::asm!("nop") }
    }
}
