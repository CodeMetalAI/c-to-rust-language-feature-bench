use std::ffi::c_void;
use std::ptr;

fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: *const i32 = ptr::null();
    let v0: *const c_void = ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: *const () = ptr::null();
    let fp1: *const () = ptr::null();
    let fp2: *const () = v0 as *const ();

    if p0 != p1 {
        std::process::exit(1);
    }
    if p0 != ptr::null() {
        std::process::exit(2);
    }

    if v0 != ptr::null() {
        std::process::exit(3);
    }
    if v0 != p0 as *const c_void {
        std::process::exit(4);
    }

    if fp0 != fp1 {
        std::process::exit(5);
    }
    if fp1 != fp2 {
        std::process::exit(6);
    }
    if fp0 != ptr::null() {
        std::process::exit(7);
    }

    if p0 != fp0 as *const i32 {
        std::process::exit(8);
    }

    std::process::exit(0);
}