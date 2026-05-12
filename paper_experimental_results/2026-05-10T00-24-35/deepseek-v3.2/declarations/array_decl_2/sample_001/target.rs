use std::mem;

type usize = u64;

static mut Y: [i32; 7] = [0; 7];
static mut X: [i32; 3] = [10, 20, 30];

fn offset_of<T, U>(_: &T, member_ptr: &U) -> usize {
    let base = 0 as *const T;
    let member = member_ptr as *const U;
    (member as usize) - (base as usize)
}

struct HoldP {
    p: *mut i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = usize::default();
    while i < n {
        s += unsafe { *p.add(i) };
        i += 1;
    }
    s
}

fn sum_arr7(a: [i32; 7]) -> i32 {
    let mut s = 0;
    let mut i = usize::default();
    while i < 7 {
        s += a[i as usize];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        p.add(1).write(*p.add(1) + 5);
        *p.add(0) + *p.add(1) + *p.add(2)
    }
}

fn main() {
    unsafe {
        Y[0] = 1;
        Y[1] = 2;
        Y[2] = 3;
        Y[3] = 4;
        Y[4] = 5;
        Y[5] = 6;
        Y[6] = 7;

        if sum_arr7(Y) != (1 + 2 + 3 + component + 5 + 6 + 7) {
            std::process::exit(1);
        }

        if sum_ptr(X.as_ptr(), 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        if mutate_through_pointer(X.as_mut_ptr()) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if X[1] != 25 {
            std::process::exit(4);
        }

        let t = Y.as_mut_ptr();
        if unsafe { *t.add(6) } != 7 {
            std::process::exit(5);
        }

        let hp = HoldP { p: Y.as_mut_ptr() };
        if unsafe { *hp.p } != 1 {
            std::process::exit(6);
        }

        let ha: *mut HoldA = Y.as_mut_ptr() as *mut HoldA;
        let ha_ref = unsafe { &*ha };
        let a_ptr = &ha_ref.a as *const _ as *const i32;
        let ha_ptr = ha as *const u8;
        let a_offset = offset_of(ha_ref, &ha_ref.a);
        if a_ptr as *const u8 != ha_ptr.add(a_offset as usize) {
            std::process::exit(7);
        }

        if unsafe { *a_ptr.add(2) } != 3 {
            std::process::exit(8);
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }

        std::process::exit(0);
    }
}