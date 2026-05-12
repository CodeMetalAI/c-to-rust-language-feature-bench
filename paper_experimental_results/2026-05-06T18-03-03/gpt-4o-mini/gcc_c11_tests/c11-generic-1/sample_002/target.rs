#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut n = 0;

    check(generic_check(n, 0));
    n += 1;

    check(generic_check(n, 0));
    n += 1;

    const CN: i32 = 0;
    check(generic_check(CN, 0));
    n += 1;

    check(generic_check(n, 0));
    n += 1;

    let a: [i32; 1] = [0];
    const CA: [i32; 1] = [0];
    check(generic_check(&a as *const _ as *const i32, 0));
    n += 1;

    check(generic_check(&CA as *const _ as *const i32, 0));
    n += 1;

    check(generic_check(f as *const (), 0));
    n += 1;

    check(generic_check(abort as *const (), 0));
    n += 1;

    let s: i16 = 0;
    check(generic_check(s, 0));
    n += 1;

    exit(0);
}

fn check(condition: bool) {
    if condition {
        abort();
    }
}

fn generic_check<T>(_: T, default: i32) -> bool {
    // Simulate _Generic behavior
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        return false;
    }
    true
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    abort();
}

extern "C" {
    fn abort() -> !;
    fn exit(code: i32) -> !;
    fn f();
}