// array_decl_2.rs

type Usize = usize;

static mut X: *const i32 = std::ptr::null();
static mut Y: [i32; 7] = [0; 7];

static BACKING: [i32; 3] = [10, 20, 30];

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
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
    unsafe {
        Y = [1, 2, 3, 4, 5, 6, 7];
        X = BACKING.as_ptr();

        if sum_arr7(&Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            std::process::exit(1);
        }

        if sum_ptr(&BACKING, 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        let mut backing_mut = BACKING;
        if mutate_through_pointer(&mut backing_mut) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if backing_mut[1] != 25 {
            std::process::exit(4);
        }

        {
            let t = &Y;
            if t[6] != 7 {
                std::process::exit(5);
            }
        }

        {
            let hp = HoldP { p: Y.as_ptr() };
            if unsafe { *hp.p } != 1 {
                std::process::exit(6);
            }

            let ha: *const HoldA = Y.as_ptr() as *const HoldA;

            if (ha as *const u8).wrapping_offset(std::mem::size_of::<HoldA>() as isize)
                != (unsafe { &(*ha).a[0] } as *const i32 as *const u8)
            {
                std::process::exit(7);
            }

            if unsafe { (*ha).a[2] } != 3 {
                std::process::exit(8);
            }
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}