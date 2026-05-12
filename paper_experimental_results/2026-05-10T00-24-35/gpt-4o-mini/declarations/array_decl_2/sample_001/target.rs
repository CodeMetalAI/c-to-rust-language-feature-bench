// array_decl_2.rs

const BACKING: [i32; 3] = [10, 20, 30];
static mut Y: [i32; 7] = [0; 7];
static X: *const i32 = BACKING.as_ptr();

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        unsafe {
            s += *p.add(i);
        }
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    a.iter().sum()
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

        if mutate_through_pointer(X as *mut i32) != (10 + 25 + 30) {
            return 3;
        }

        if BACKING[1] != 25 {
            return 4;
        }

        {
            let t = &Y as *const _ as *const i32;
            if unsafe { *t.add(6) } != 7 {
                return 5;
            }
        }

        {
            let hp = HoldP { p: &Y };
            if unsafe { *hp.p } != 1 {
                return 6;
            }

            let ha = &Y as *const _ as *const HoldA;

            if (ha as *const u8) + std::mem::size_of::<HoldA>() as isize != (ha as *const u8).add(8) {
                return 7;
            }

            if unsafe { (*ha).a[2] } != 3 {
                return 8;
            }
        }

        if Y[0] != 1 {
            return 9;
        }
    }

    0
}

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}