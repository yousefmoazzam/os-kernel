#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_byte(b'H');
    vga_buffer::WRITER.lock().write_string("ello, ");
    vga_buffer::WRITER.lock().write_string("wörld!");
    vga_buffer::WRITER.lock().write_new_line();
    write!(vga_buffer::WRITER.lock(), "Testing out formatting: {}", 3,).unwrap();
    println!(
        "Printing with a newline at the end, plus formatting: {}",
        5.0
    );
    print!("Print");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
