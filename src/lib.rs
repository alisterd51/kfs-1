#![no_std]
#![no_main]

mod driver;
mod io;
mod string;

use core::panic::PanicInfo;
use driver::keyboard;
use driver::tty;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    tty::clear();

    #[cfg(debug_assertions)]
    tty::test_colors();

    println!("{}42{}", tty::FG_GREEN, tty::FG_RESET);

    let mut keyboard = keyboard::ps2::Keyboard::new();

    loop {
        keyboard.get_input();
        keyboard.interpret_to_vga_text_mode();
    }
}
