#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

fn check(n: i32, counter: &mut i32) {
    if n != 0 {
        abort();
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut n = 0;

    check(0, &mut n);
    check(n, &mut n);

    check(0, &mut n);
    check(n, &mut n);

    const CN: i32 = 0;
    check(0, &mut n);
    check(n, &mut n);
    check(0, &mut n);
    check(n, &mut n);

    let a: [i32; 1] = [0];
    const CA: [i32; 1] = [0];
    check(0, &mut n);
    check(n, &mut n);
    check(0, &mut n);
    check(n, &mut n);

    check(0, &mut n);
    check(n, &mut n);

    let s: i16 = 0;
    check(0, &mut n);
    check(n, &mut n);

    exit(0);
}

fn exit(code: i32) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    abort()
}