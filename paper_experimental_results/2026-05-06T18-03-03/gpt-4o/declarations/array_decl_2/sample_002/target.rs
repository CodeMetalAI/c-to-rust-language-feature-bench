// array_decl_2.rs
type Usize = usize;

static mut X: Option<&'static mut [i32; 3]> = None;
static mut Y: [i32; 7] = [0; 7];

struct HoldP<'a> {
    p: &'a [i32],
}

struct HoldA<'a> {
    tag: i32,
    a: &'a [i32],
}

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i];
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

fn mutate_through_pointer(p: &mut [i32; 3]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() -> i32 {
    unsafe {
        let backing: &mut [i32; 3] = &mut [10, 20, 30];
        X = Some(backing);

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

        if let Some(x) = X {
            if sum_ptr(x, 3) != (10 + 20 + 30) {
                return 2;
            }

            if mutate_through_pointer(x) != (10 + 25 + 30) {
                return 3;
            }

            if backing[1] != 25 {
                return 4;
            }
        }

        {
            let t = &Y;
            if t[6] != 7 {
                return 5;
            }
        }

        {
            let hp = HoldP { p: &Y };
            if hp.p[0] != 1 {
                return 6;
            }

            let ha = HoldA { tag: 0, a: &Y };

            if ha.a.as_ptr() as Usize != (&ha as *const HoldA as Usize + std::mem::size_of::<i32>()) {
                return 7;
            }

            if ha.a[2] != 3 {
                return 8;
            }
        }

        if Y[0] != 1 {
            return 9;
        }

        0
    }
}