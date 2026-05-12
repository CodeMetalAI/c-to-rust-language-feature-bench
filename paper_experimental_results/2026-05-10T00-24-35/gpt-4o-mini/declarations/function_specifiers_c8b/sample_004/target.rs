#![no_std]
#![no_main]

use core::panic::PanicInfo;

static mut G: i32 = 0;

#[no_mangle]
pub extern "C" fn die_if(x: i32) -> ! {
    if x == 7 && unsafe { G } == 9 {
        core::process::exit(0);
    } else {
        core::process::exit(2);
    }
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    unsafe {
        G = 9;
    }
    die_if(7);
    3
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}