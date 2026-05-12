#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn die0() -> ! {
    loop {}
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
    let _ = v; // to avoid warning
    1
}