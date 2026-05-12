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
    check(generic_int(CN, 0, n += 1));
    check(n);
    check(generic_int(CN as i32, 0, n += 1));
    check(n);

    let a: [i32; 1] = [0];
    const CA: [i32; 1] = [0];
    check(generic_pointer(&a as *const _ as *const i32, 0, n += 1));
    check(n);
    check(generic_pointer(&CA as *const _ as *const i32, n += 1, 0));
    check(n);

    check(generic_function(f as fn(), 0, n += 1));
    check(n);

    check(generic_function(abort as fn(), 0, n += 1));
    check(n);

    let s: i16 = 0;
    check(generic_short(s, 0, n += 1));
    check(n);

    exit(0);
}

fn check(n: i32) {
    if n != 0 {
        abort();
    }
}

fn generic_int<T: Into<i32>>(value: T, if_int: i32, if_default: i32) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        if_int
    } else {
        if_default
    }
}

fn generic_double<T: Into<f64>>(value: T, if_double: i32, if_default: i32) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() {
        if_double
    } else {
        if_default
    }
}

fn generic_pointer<T>(_: *const T, if_ptr: i32, if_const_ptr: i32) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        if_ptr
    } else {
        if_const_ptr
    }
}

fn generic_function<T>(_: T, if_func: i32, if_default: i32) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<fn()>() {
        if_func
    } else {
        if_default
    }
}

fn generic_short<T: Into<i16>>(value: T, if_short: i32, if_int: i32) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i16>() {
        if_short
    } else {
        if_int
    }
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