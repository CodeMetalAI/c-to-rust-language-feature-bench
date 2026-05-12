use std::ptr;
use std::process::exit;

fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: *const i32 = ptr::null();
    let v0: *const () = ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: Option<fn(i32) -> i32> = None;
    let fp1: Option<fn(i32) -> i32> = None;
    let fp2: Option<fn(i32) -> i32> = if v0.is_null() {
        None
    } else {
        Some(f as fn(i32) -> i32)
    };

    if p0 != p1 {
        exit(1);
    }
    if p0 != ptr::null() {
        exit(2);
    }

    if v0 != ptr::null() {
        exit(3);
    }
    if v0 != p0 as *const () {
        exit(4);
    }

    if fp0 != fp1 {
        exit(5);
    }
    if fp1 != fp2 {
        exit(6);
    }
    if fp0 != None {
        exit(7);
    }

    let fp0_ptr: *const i32 = match fp0 {
        Some(func) => func as *const i32,
        None => ptr::null(),
    };

    if p0 != fp0_ptr {
        exit(8);
    }

    exit(0);
}