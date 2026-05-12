struct HoldP {
    p: *mut i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 7],
}

static mut BACKING: [i32; 3] = [10, 20, 30];
static mut X: *mut i32 = unsafe { &mut BACKING[0] };
static mut Y: [i32; 7] = [0; 7];

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
        *p + *p.add(1) + *p.add(2)
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

        if sum_ptr(X, 3) != (10 + 20 + 30) {
            return 2;
        }

        if mutate_through_pointer(X) != (10 + 25 + 30) {
            return 3;
        }

        if BACKING[1] != 25 {
            return 4;
        }

        {
            let t = &Y as *const i32;
            if *t.add(6) != 7 {
                return 5;
            }
        }

        {
            let mut hp = HoldP { p: &mut Y[0] };
            if *hp.p != 1 {
                return 6;
            }

            let ha = &Y as *const i32 as *const HoldA;
            let ha_a_ptr = &(*ha).a[0] as *const i32 as *const u8;
            let ha_ptr = ha as *const u8;
            let offset = std::mem::size_of::<i32>();

            if ha_a_ptr != ha_ptr.add(offset) {
                return 7;
            }

            if (*ha).a[2] != 3 {
                return 8;
            }
        }

        if Y[0] != 1 {
            return 9;
        }
    }

    0
}