// We do not want to depend on _any_ libraries.
#![no_std]

// We do not want the C and then the Rust runtime
// to look for a `main` function here.
#![no_main]

// We're not using anything in this crate yet
// so in order to force the Rust compiler to link it
// we have to do the following in addition to referencing
// it in Cargo.toml.
extern crate rlibc;

// This is including our own module
mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;

// This annotation specifies that the function
// below is called upon panic.
#[panic_handler]
// This function is never actually called.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// This annotation instructs the compiler to not mangle
// the name of the function so that we can insure that
// `_start` will be available to the linker.
#[no_mangle]
// This function is called by the Rust runtime
// instead of `main`.
pub extern "C" fn _start() -> ! {
    let mut writer = vga_buffer::WRITER.lock();
    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();

    loop {}
}
