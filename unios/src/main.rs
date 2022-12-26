#![no_std]
#![no_main]

mod vga_buf;


use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write;

use self::vga_buf::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {

    let mut driver = VGADriver::new(Color::Brown, Color::White, Alignment::Center);

    for i in 0..100
    {
        write!(driver, "Number {}\n", i);
    }

    loop {}
}