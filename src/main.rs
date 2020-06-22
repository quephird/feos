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

// This annotation instructs the compiler to not mangle
// the name of the function so that we can insure that
// `_start` will be available to the linker.
#[no_mangle]
// This function is called by the Rust runtime
// instead of `main`.
pub extern "C" fn _start() -> ! {
    loop {}
}
