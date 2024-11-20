#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello, world!";
static VGA_BUFFER_START: u32 = 0xB8000;
static LIGHT_CYAN: u8 = 0xB;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = VGA_BUFFER_START as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = LIGHT_CYAN;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
