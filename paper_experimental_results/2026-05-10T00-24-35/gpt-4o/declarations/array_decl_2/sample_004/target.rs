// main.rs

type Usize = usize;

static mut Y: [i32; 7] = [0; 7];

static BACKING: [i32; 3] = [10, 20, 30];

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

fn mutate_through_pointer(p: &mut [i32; 3]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() -> i32 {
    unsafe {
        Y = [1, 2, 3, 4, 5, 6, 7];
    }

    unsafe {
        if sum_arr7(&Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            return 1;
        }
    }

    if sum_ptr(&BACKING, 3) != (10 + 20 + 30) {
        return 2;
    }

    let mut backing_clone = BACKING;
    if mutate_through_pointer(&mut backing_clone) != (10 + 25 + 30) {
        return 3;
    }

    if backing_clone[1] != 25 {
        return 4;
    }

    unsafe {
        let t = &Y;
        if t[6] != 7 {
            return 5;
        }
    }

    {
        struct HoldP<'a> {
            p: &'a [i32],
        }

        struct HoldA<'a> {
            tag: i32,
            a: &'a [i32],
        }

        let hp = HoldP { p: unsafe { &Y } };
        if hp.p[0] != 1 {
            return 6;
        }

        let ha = HoldA {
            tag: 0,
            a: unsafe { &Y },
        };

        if ha.a.as_ptr() as Usize != (&ha.a[0] as *const i32) as Usize {
            return 7;
        }

        if ha.a[2] != 3 {
            return 8;
        }
    }

    unsafe {
        if Y[0] != 1 {
            return 9;
        }
    }

    0
}