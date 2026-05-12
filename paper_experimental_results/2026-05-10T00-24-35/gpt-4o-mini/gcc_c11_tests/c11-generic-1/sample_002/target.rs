#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn exit(_: i32) -> ! {
    loop {}
}

fn check(n: &mut i32, condition: bool) {
    if condition {
        abort();
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut n = 0;

    check(&mut n, n += 1 == 0);
    check(&mut n, n != 0);

    check(&mut n, n.is::<f64>());
    n += 1; // Simulate the post-increment
    check(&mut n, n != 0);

    const cn: i32 = 0;
    check(&mut n, cn.is::<i32>());
    check(&mut n, n != 0);
    check(&mut n, (cn as i32).is::<i32>());
    check(&mut n, n != 0);

    let mut a: [i32; 1] = [0];
    const ca: [i32; 1] = [0];
    check(&mut n, a.as_ptr().is::<*const i32>());
    n += 1;
    check(&mut n, n != 0);
    check(&mut n, ca.as_ptr().is::<*const i32>());
    n += 1;
    check(&mut n, n != 0);

    fn f() {}
    check(&mut n, f as extern "C" fn() == f);
    check(&mut n, n != 0);

    check(&mut n, abort as extern "C" fn() == abort);
    check(&mut n, n != 0);

    let mut s: i16 = 0;
    check(&mut n, s.is::<i16>());
    check(&mut n, n != 0);

    exit(0);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}