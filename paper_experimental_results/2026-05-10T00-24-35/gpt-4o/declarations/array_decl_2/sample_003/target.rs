// array_decl_2.rs
type Usize = usize;

static mut X: Option<&'static mut [i32]> = None;
static mut Y: [i32; 7] = [0; 7];

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0], // Flexible array member equivalent
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
    a.iter().sum()
}

fn mutate_through_pointer(p: &mut [i32; 3]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() {
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
            std::process::exit(1);
        }

        if let Some(x) = X {
            if sum_ptr(x, 3) != (10 + 20 + 30) {
                std::process::exit(2);
            }

            if mutate_through_pointer(backing) != (10 + 25 + 30) {
                std::process::exit(3);
            }
        }

        if backing[1] != 25 {
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

            let ha: &HoldA = unsafe { &*(Y.as_ptr() as *const HoldA) };
            let offset_of_a = std::mem::size_of::<i32>();
            let ha_a_offset = ha.a.as_ptr() as usize - ha as *const _ as usize;
            if ha_a_offset != offset_of_a {
                std::process::exit(7);
            }

            if unsafe { *ha.a.as_ptr().add(2) } != 3 {
                std::process::exit(8);
            }
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}