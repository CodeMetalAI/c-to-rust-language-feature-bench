fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: *mut i32 = std::ptr::null_mut();
    let v0: *mut () = std::ptr::null_mut();
    let p1: *mut i32 = v0 as *mut i32;

    let fp0: fn(i32) -> i32 = || { 0 };
    let fp1: fn(i32) -> i32 = || { 0 };
    let fp2: fn(i32) -> i32 = unsafe { std::ptr::read_volatile(v0 as *const fn(i32) -> i32) };

    if p0 != p1 {
        std::process::exit(1);
    }
    if p0 != std::ptr::null_mut() as *mut i32 {
        std::process::exit(2);
    }

    if v0 != std::ptr::null_mut() as *mut () {
        std::process::exit(3);
    }
    if v0 != p0 as *mut () {
        std::process::exit(4);
    }

    if fp0 != fp1 {
        std::process::exit(5);
    }
    if fp1 != fp2 {
        std::process::exit(6);
    }
    if fp0 != || { 0 } {
        std::process::exit(7);
    }

    if p0 != &fp0 as *const fn(i32) -> i32 as *mut i32 {
        std::process::exit(8);
    }

    std::process::exit(0);
}