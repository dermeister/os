#![no_std]

use core::panic::PanicInfo;

#[macro_use]
mod vga;
mod spinlock;
mod lazy;

#[no_mangle]
pub extern "C" fn main() -> ! {
    for i in 0.. {
        println!("Next number is: {}", i);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
