type usize = u64;

static mut BACKING: [i32; 3] = [10, 20, 30];
static mut X: *const i32 = BACKING.as_ptr();

static mut Y: [i32; 7] = [0; 7];

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i as usize];
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

        if sum_ptr(&*X, 3) != (10 + 20 + 30) {
            return 2;
        }

        if mutate_through_pointer(&mut *X) != (10 + 25 + 30) {
            return 3;
        }

        if BACKING[1] != 25 {
            return 4;
        }

        {
            let t = &Y;
            if t[6] != 7 {
                return 5;
            }
        }

        {
            let hp = &mut Y as *mut _ as *mut HoldP;
            let ha = hp as *mut HoldA;

            if ha as *const _ as usize + std::mem::size_of::<HoldA>() as usize != &(*ha).a as *const _ as usize {
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

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}