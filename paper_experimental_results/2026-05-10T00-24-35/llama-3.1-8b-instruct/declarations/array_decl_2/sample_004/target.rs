#![allow(unused)]

use std::mem;

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += unsafe { *p.offset(i as isize) };
        i += 1;
    }
    s
}

fn sum_arr7(arr: [i32; 7]) -> i32 {
    let mut s = 0;
    for i in 0..7 {
        s += arr[i];
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
    let y: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    if sum_arr7(y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        panic!("Sum of array 7 is incorrect");
    }

    let backing: [i32; 3] = [10, 20, 30];
    let x = &backing as *const i32;
    if sum_ptr(x, 3) != (10 + 20 + 30) {
        panic!("Sum of pointer is incorrect");
    }

    if mutate_through_pointer(x.offset(0)) != (10 + 25 + 30) {
        panic!("Mutation through pointer is incorrect");
    }

    if unsafe { backing[1] } != 25 {
        panic!("backing[1] is incorrect");
    }

    let y = &y as *const i32;
    {
        let t = unsafe { y };
        if t.offset(6) != y.offset(6) {
            panic!("panic 5");
        }
    }

    {
        let hp = HoldP { p: y };
        if unsafe { *hp.p.offset(0) } != 1 {
            panic!("panic 6");
        }

        let ha = unsafe { (y as *const i32) as *const HoldA };
        if unsafe { (ha as *const u8).offset(OFFSETOF!(HoldA, a) as isize) } != unsafe { ha as *const u8 }.offset(0) {
            panic!("panic 7");
        }

        if unsafe { ha.as_ref().a[2] } != 3 {
            panic!("panic 8");
        }
    }

    if unsafe { *y.offset(0) } != 1 {
        panic!("panic 9");
    }
}

const OFFSETOF: usize = mem::align_of::<HoldA>();

fn OFFSETOF!(type, member) -> usize {
    std::mem::size_of::<type>() - std::mem::align_of::<type>()
}