#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn stop() -> ! {
    let mut x = 0;
    loop {
        x += 1;
    }
}

fn f() -> ! {
    stop();
}

fn g(i: i32) -> ! {
    if i > 0 {
        stop();
    }
}

fn main() -> ! {
    if true {
        f();
    }

    g(1);
}