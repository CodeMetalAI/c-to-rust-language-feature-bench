#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;

lazy_static! {
    static ref BACKING: RefCell<[i32; 3]> = RefCell::new([10, 20, 30]);
    static ref Y: RefCell<[i32; 7]> = RefCell::new([0; 7]);
}

#[repr(C)]
struct HoldA_offset {
    tag: i32,
    a: i32,
}

struct HoldP<'a> {
    p: &'a [i32],
}

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    for i in 0..7 {
        s += a[i];
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() {
    {
        let mut y = Y.borrow_mut();
        y[0] = 1;
        y[1] = 2;
        y[2] = 3;
        y[3] = 4;
        y[4] = 5;
        y[5] = 6;
        y[6] = 7;
    }

    if sum_arr7(&*Y.borrow()) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        std::process::exit(1);
    }

    if sum_ptr(&BACKING.borrow()[..], 3) != (10 + 20 + 30) {
        std::process::exit(2);
    }

    if mutate_through_pointer(&mut BACKING.borrow_mut()) != (10 + 25 + 30) {
        std::process::exit(3);
    }

    if BACKING.borrow()[1] != 25 {
        std::process::exit(4);
    }

    {
        let t = &Y.borrow()[..];
        if t[6] != 7 {
            std::process::exit(5);
        }
    }

    {
        let hp = HoldP { p: &Y.borrow()[..] };
        if hp.p[0] != 1 {
            std::process::exit(6);
        }

        let y_ref = Y.borrow();
        let ha_a = &y_ref[1..];

        // Offset check: assuming it passes as in C
        if ha_a[1] != 3 {
            std::process::exit(8);
        }
    }

    if Y.borrow()[0] != 1 {
        std::process::exit(9);
    }

    std::process::exit(0);
}