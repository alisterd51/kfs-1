#![no_std]
#![no_main]
#![feature(c_size_t)]

mod driver;
mod gdt;
mod io;
mod print_kernel_stack;
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
    gdt::init_gdt();

    tty::clear();

    #[cfg(debug_assertions)]
    tty::test_colors();

    println!("{}42{}", tty::FG_GREEN, tty::FG_RESET);

    print_kernel_stack::print_kernel_stack(0);

    let mut keyboard = keyboard::ps2::Keyboard::new();

    loop {
        keyboard.get_input();
        keyboard.interpret_to_vga_text_mode();
    }
}
