// We do not want to depend on _any_ libraries.
#![no_std]

// We do not want the C and then the Rust runtime
// to look for a `main` function here.
#![no_main]

use core::panic::PanicInfo;

// This annotation specifies that the function
// below is called upon panic.
#[panic_handler]
// This function is never actually called.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// This annotation instructs the compiler to not mangle
// the name of the function so that we can insure that
// `_start` will be available to the linker.
#[no_mangle]
// This function is called by the Rust runtime
// instead of `main`.
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
