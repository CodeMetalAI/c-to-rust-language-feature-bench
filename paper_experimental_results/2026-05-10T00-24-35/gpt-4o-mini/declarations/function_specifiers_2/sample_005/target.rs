#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    if true {
        f();
    }

    g(1);

    loop {}
}

fn f() -> ! {
    stop();
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop();
    }
}

fn stop() -> ! {
    let mut x = 0;
    loop {
        x += 1;
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}