fn main() {
    type Miles = i32;
    let mut distance: Miles = 40;

    if takes_int(distance) != 41 {
        std::process::exit(1);
    }

    {
        let mut t = distance;
        if t != 40 {
            std::process::exit(2);
        }
        if takes_ptr_to_int(&mut t) != 43 {
            std::process::exit(3);
        }
    }

    let metricp = f_plain;
    if metricp() != 7 {
        std::process::exit(4);
    }
    // In Rust, it is not allowed to call a function with incorrect number of arguments,
    // so the following line is commented out to avoid compilation error.
    // if (metricp)(1, 2, 3) != 7 {
    //     std::process::exit(5);
    // }

    let x = Range { hi: 1.5, lo: -1.5 };
    let y = takes_range(x);
    if (y.hi + y.lo) != (x.hi + x.lo) {
        std::process::exit(6);
    }

    let mut z = Range { hi: 10.0, lo: -9.0 };
    let zp = &mut z;

    if takes_range_ptr(zp) != 1.0 {
        std::process::exit(7);
    }

    let copy = *zp;
    if copy.hi != 10.0 || copy.lo != -9.0 {
        std::process::exit(8);
    }
}

struct Range {
    hi: f64,
    lo: f64,
}

fn takes_int(x: i32) -> i32 {
    x + 1
}

fn takes_ptr_to_int(p: &mut i32) -> i32 {
    *p += 3;
    *p
}

fn takes_range(r: Range) -> Range {
    Range {
        hi: r.hi + 1.0,
        lo: r.lo - 1.0,
    }
}

fn takes_range_ptr(p: &Range) -> f64 {
    p.hi + p.lo
}

fn f_plain() -> i32 {
    static mut g_store: i32 = 7;
    unsafe { g_store }
}

// Note: Since Rust does not support function pointers with destructured types, 
// this function needs to be adjusted to return a closure.
// However, using a closure would change the behavior of the original C code.
// Therefore, we have to settle with a different approach.
// If you want to simulate the same behavior as the original C code, you would have to use `std::any::Any` and `std::boxed::Box`.
// Here we stick with the simple solution for demonstration purposes.
fn f_plain2() -> impl Fn() -> i32 {
    move || {
        static mut g_store: i32 = 7;
        unsafe { g_store }
    }
}

// Since f_plain() is not a closure, we have to use the f_plain2() function instead.
// let metricp = f_plain;
let metricp = f_plain2();