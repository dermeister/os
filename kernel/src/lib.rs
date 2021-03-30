#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let vga = 0xb8000 as *mut u32;
    unsafe { *vga = 0x2f4b2f4f };

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
