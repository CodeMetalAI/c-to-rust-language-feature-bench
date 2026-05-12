#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() {
    let mut n = 0;

    check(if n == 0 { 0 } else { unreachable!() });
    check(n);

    check(if std::any::TypeId::of::<f64>() == std::any::TypeId::of::<i32>() {
        n += 1;
        n
    } else { 0 });
    check(n);

    const CN: i32 = 0;
    check(if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>() {
        0
    } else {
        n += 1;
        n
    });
    check(n);
    check(if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>() {
        0
    } else {
        n += 1;
        n
    });
    check(n);

    let a: [i32; 1] = [0];
    let ca: [i32; 1] = [0];
    check(if std::any::TypeId::of::<&[i32; 1]>() == std::any::TypeId::of::<*const i32>() {
        0
    } else {
        n += 1;
        n
    });
    check(n);
    check(if std::any::TypeId::of::<&[i32; 1]>() == std::any::TypeId::of::<*const i32>() {
        0
    } else {
        n += 1;
        n
    });
    check(n);

    check(if std::any::TypeId::of::<fn()>() == std::any::TypeId::of::<fn()> {
        0
    } else {
        n += 1;
        n
    });
    check(n);

    check(if std::any::TypeId::of::<fn()>() == std::any::TypeId::of::<fn()> {
        0
    } else {
        n += 1;
        n
    });
    check(n);

    let s: i16 = 0;
    check(if std::any::TypeId::of::<i16>() == std::any::TypeId::of::<i16>() {
        0
    } else {
        n += 1;
        n
    });
    check(n);

    exit(0);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

fn exit(code: i32) -> ! {
    // This is where we would terminate the program
    // For a standalone Rust program, we can use std::process::exit
    core::process::abort();
}

fn abort() -> ! {
    // This is where we would abort the program
    core::process::abort();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    abort();
}