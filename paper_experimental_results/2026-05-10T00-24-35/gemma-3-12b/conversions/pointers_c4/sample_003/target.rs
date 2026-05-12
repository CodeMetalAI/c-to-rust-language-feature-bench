fn f(x: i32) -> i32 {
    x
}

fn main() {
    let mut p0: *mut i32 = std::ptr::null_mut();
    let mut v0: *mut () = std::ptr::null_mut();
    let mut p1: *mut i32 = v0 as *mut i32;

    let mut fp0: fn(i32) -> i32 = || 0;
    let mut fp1: fn(i32) -> i32 = || 0;
    let mut fp2: fn(i32) -> i32 = unsafe { std::mem::transmute(v0) };

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
    if fp0 != || 0 {
        std::process::exit(7);
    }

    if p0 != fp0 as *mut i32 {
        std::process::exit(8);
    }

    std::process::exit(0);
}