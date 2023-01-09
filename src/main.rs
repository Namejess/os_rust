#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga_buffer;


static HELLO: &[u8] = b"Mon premier Kernel en Rust!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
