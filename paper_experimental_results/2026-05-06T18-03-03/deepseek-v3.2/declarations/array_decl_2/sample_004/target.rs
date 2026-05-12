use std::mem;

type usize = u64;

static mut X: *const i32 = std::ptr::null();
static mut Y: [i32; 7] = [0; 7];

static mut BACKING: [i32; 3] = [10, 20, 30];

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}

fn offset_of<T, U>(_: &T, member: *const U) -> usize {
    let base = 0 as *const T;
    let member_ptr = unsafe { base.cast::<u8>().offset(member.offset_from(base.cast())) };
    member_ptr as usize
}

fn sum_ptr(p: *const i32, n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += unsafe { *p.offset(i as isize) };
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < 7 {
        s += a[i as usize];
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
    unsafe {
        X = BACKING.as_ptr();
        Y[0] = 1;
        Y[1] = 2;
        Y[2] = 3;
        Y[3] = 4;
        Y[4] = 5;
        Y[5] = 6;
        Y[6] = 7;

        if sum_arr7(&Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            std::process::exit(1);
        }

        if sum_ptr(X, 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        if mutate_through_pointer(BACKING.as_mut_ptr()) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if BACKING[1] != 25 {
            std::process::exit(4);
        }

        {
            let t = Y.as_ptr();
            if unsafe { *t.offset(6) } != 7 {
                std::process::exit(5);
            }
        }

        {
            let mut hp = HoldP { p: std::ptr::null() };
            hp.p = Y.as_ptr();
            if unsafe { *hp.p.offset(0) } != 1 {
                std::process::exit(6);
            }

            let ha = Y.as_ptr() as *const HoldA;
            let ha_a_ptr = unsafe { &(*ha).a } as *const _ as *const i32;
            let expected_offset = offset_of(&*ha, ha_a_ptr as *const i32);

            let ha_ptr = ha as *const u8;
            let a_ptr = ha_a_ptr as *const u8;
            if a_ptr != unsafe { ha_ptr.add(expected_offset as usize) } {
                std::process::exit(7);
            }

            if unsafe { *ha_a_ptr.offset(2) } != 3 {
                std::process::exit(8);
            }
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }

        std::process::exit(0);
    }
}