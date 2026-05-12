#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut n = 0;

    check(generic_int(n += 1, 0));
    check(n);

    check(generic_double(n, n += 1, 0));
    check(n);

    const CN: i32 = 0;
    check(generic_int(CN, n += 1));
    check(n);
    check(generic_int(CN as i32, n += 1));
    check(n);

    let a: [i32; 1] = [0];
    const CA: [i32; 1] = [0];
    check(generic_pointer(&a as *const _ as *const i32, n += 1));
    check(n);
    check(generic_pointer(&CA as *const _ as *const i32, n += 1));
    check(n);

    check(generic_function(f as *const () as *const fn(), n += 1));
    check(n);

    check(generic_function(abort as *const () as *const fn(), n += 1));
    check(n);

    let s: i16 = 0;
    check(generic_short(s, n += 1));
    check(n);

    exit(0);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

fn generic_int<T: Into<i32>>(value: T, default: i32) -> i32 {
    default
}

fn generic_double<T: Into<i32>>(value: T, increment: i32, default: i32) -> i32 {
    default
}

fn generic_pointer<T>(value: *const T, increment: i32) -> i32 {
    increment
}

fn generic_function<T>(value: *const T, increment: i32) -> i32 {
    increment
}

fn generic_short<T: Into<i32>>(value: T, increment: i32) -> i32 {
    increment
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn exit(code: i32) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

fn f() {}