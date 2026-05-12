#![no_std]
#![no_main]

use core::panic::PanicInfo;

static mut G: i32 = 0;

#[no_mangle]
pub extern "C" fn die_if(x: i32) -> ! {
    if x == 7 && unsafe { G } == 9 {
        exit(0);
    } else {
        exit(2);
    }
}

fn exit(code: i32) -> ! {
    // Simulate _Exit by not returning to the caller
    core::intrinsics::abort();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        G = 9;
    }
    die_if(7);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(1);
}