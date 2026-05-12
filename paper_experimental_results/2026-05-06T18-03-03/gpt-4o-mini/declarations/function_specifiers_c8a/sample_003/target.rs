#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _exit(code: i32) -> ! {
    loop {}
}

fn die0() -> ! {
    _exit(0);
}

fn pick(x: i32) -> i32 {
    if x != 0 {
        123
    } else {
        die0()
    }
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let v = pick(0);
    let _ = v; // suppress unused variable warning
    1
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}