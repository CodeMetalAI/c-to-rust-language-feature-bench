// array_decl_2.rs
use std::ptr;

const BACKING: [i32; 3] = [10, 20, 30];
static mut X: *mut i32 = BACKING.as_mut_ptr();

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += *p.offset(i as isize);
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < a.len() {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.offset(1) += 5;
        *p.offset(0) + *p.offset(1) + *p.offset(2)
    }
}

fn main() {
    let mut y = [1, 2, 3, 4, 5, 6, 7];
    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        panic!("sum_arr7 failed");
    }

    if sum_ptr(X, 3) != (10 + 20 + 30) {
        panic!("sum_ptr failed");
    }

    if mutate_through_pointer(X) != (10 + 25 + 30) {
        panic!("mutate_through_pointer failed");
    }

    if unsafe { *X.offset(1) } != 25 {
        panic!("backing[1] failed");
    }

    {
        let t = &y;
        if t[6] != 7 {
            panic!("t[6] failed");
        }
    }

    {
        let hp = HoldP { p: y.as_mut_ptr() };
        if unsafe { *hp.p.offset(0) } != 1 {
            panic!("hp.p[0] failed");
        }

        let ha = (ha as *mut HoldA).as_mut_ptr();
        if unsafe { (ha as *const i8).offset(OFFSETOF!(HoldA, a)) as *const i32 } != ha {
            panic!("ha->a failed");
        }

        if unsafe { *ha.offset(2) } != 3 {
            panic!("ha->a[2] failed");
        }
    }

    if y[0] != 1 {
        panic!("y[0] failed");
    }
}

struct HoldP {
    p: *mut i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 3],
}

const fn OFFSETOF<T, U>(_: T) -> usize {
    std::mem::size_of::<U>() as usize
}