#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let v = pick(0);
    let _ = v; // to avoid unused variable warning
    exit(1);
}

fn die0() -> ! {
    exit(0);
}

fn pick(x: i32) -> i32 {
    if x != 0 {
        123
    } else {
        die0()
    }
}

fn exit(code: i32) -> ! {
    // In a real application, you'd use a suitable exit mechanism
    // For this example, we'll just loop infinitely
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}