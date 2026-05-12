use std::process;

fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: i32 = 1;
    let px: *const () = &x as *const i32 as *const ();
    let pf: *const () = f as *const ();
    let n1: *const () = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let n2: *const () = v0;
    let nf1: *const () = std::ptr::null();
    let vf0: *const () = std::ptr::null();
    let nf2: *const () = vf0;
    if !n1.is_null() {
        process::exit(1);
    }
    if !n2.is_null() {
        process::exit(2);
    }
    if !v0.is_null() {
        process::exit(3);
    }
    if n1 == px {
        process::exit(4);
    }
    if n2 == px {
        process::exit(5);
    }
    if !nf1.is_null() {
        process::exit(6);
    }
    if !nf2.is_null() {
        process::exit(7);
    }
    if nf1 == pf {
        process::exit(8);
    }
    if nf2 == pf {
        process::exit(9);
    }
    let vx: *const () = px;
    let vf: *const () = pf;
    if vx.is_null() {
        process::exit(10);
    }
    if vf.is_null() {
        process::exit(11);
    }
    if vx == v0 {
        process::exit(12);
    }
    if vf == vf0 {
        process::exit(13);
    }
    process::exit(0);
}