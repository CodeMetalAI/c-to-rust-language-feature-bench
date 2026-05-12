use std::mem;

type usize = std::os::raw::c_ulong;

static mut X: [i32; 3] = [10, 20, 30];
static mut Y: [i32; 7] = [0, 0, 0, 0, 0, 0, 0];

const fn offset_of<T, U>(_: &T, member: *const U) -> usize {
    let base = 0 as *const T;
    let member_ptr = member as *const usize;
    let base_ptr = base as *const usize;
    (member_ptr as usize) - (base_ptr as usize)
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
    let mut i = 0;
    while i < n {
        s += unsafe { *p.add(i) };
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < 7 {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.add(1) += 5;
        *p.add(0) + *p.add(1) + *p.add(2)
    }
}

fn main() -> i32 {
    unsafe {
        Y[0] = 1;
        Y[1] = 2;
        Y[2] = 3;
        Y[3] = 4;
        Y[4] = 5;
        Y[5] = 6;
        Y[6] = 7;

        if sum_arr7(&Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            return 1;
        }

        if sum_ptr(X.as_ptr(), 3) != (10 + 20 + 30) {
            return 2;
        }

        if mutate_through_pointer(X.as_mut_ptr()) != (10 + 25 + 30) {
            return 3;
        }

        if X[1] != 25 {
            return 4;
        }

        {
            let t = Y.as_mut_ptr();
            if unsafe { *t.add(6) } != 7 {
                return 5;
            }
        }

        {
            let mut hp = HoldP { p: Y.as_mut_ptr() };
            if unsafe { *hp.p } != 1 {
                return 6;
            }

            let ha = Y.as_mut_ptr() as *mut HoldA;
            let ha_a_ptr = unsafe { &(*ha).a } as *const [i32; 0] as *const usize;
            let ha_ptr = ha as *const usize;
            let computed_offset = (ha_a_ptr as usize) - (ha_ptr as usize);
            let expected_offset = offset_of(&HoldA { tag: 0 }, &HoldA { tag: 0 }.a);
            if computed_offset != expected_offset {
                return 7;
            }

            if unsafe { (*ha).a.as_ptr().add(2).read() } != 3 {
                return 8;
            }
        }

        if Y[0] != 1 {
            return 9;
        }

        0
    }
}