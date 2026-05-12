use std::ptr;

fn f(x: i32) -> i32 { x + 1 }

fn main() {
    let x = 1;
    let px: *const i32 = &x;
    let pf: fn(i32) -> i32 = f;
    let n1: *const i32 = ptr::null();
    let v0: *const () = ptr::null();
    let n2: *const i32 = v0 as *const i32;
    let nf1: *const fn(i32) -> i32 = ptr::null();
    let vf0: *const () = ptr::null();
    let nf2: *const fn(i32) -> i32 = vf0 as *const fn(i32) -> i32;

    if !n1.is_null() {
        std::process::exit(1);
    }
    if !n2.is_null() {
        std::process::exit(2);
    }
    if !v0.is_null() {
        std::process::exit(3);
    }
    if n1 == px {
        std::process::exit(4);
    }
    if n2 == px {
        std::process::exit(5);
    }
    if !nf1.is_null() {
        std::process::exit(6);
    }
    if !nf2.is_null() {
        std::process::exit(7);
    }
    if nf1 == pf as *const fn(i32) -> i32 {
        std::process::exit(8);
    }
    if nf2 == pf as *const fn(i32) -> i32 {
        std::process::exit(9);
    }
    let vx: *const () = px as *const ();
    let vf: *const () = pf as *const fn(i32) -> i32 as *const ();
    if vx.is_null() {
        std::process::exit(10);
    }
    if vf.is_null() {
        std::process::exit(11);
    }
    if vx == v0 {
        std::process::exit(12);
    }
    if vf == vf0 {
        std::process::exit(13);
    }
    std::process::exit(0);
}