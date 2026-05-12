struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32],
}

static mut BACKING: [i32; 3] = [10, 20, 30];
static mut Y: [i32; 7] = [0; 7];

fn sum_ptr(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    a.iter().sum()
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p.iter().sum()
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

        if sum_arr7(&Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            std::process::exit(1);
        }

        if sum_ptr(&BACKING) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        if mutate_through_pointer(&mut BACKING) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if BACKING[1] != 25 {
            std::process::exit(4);
        }

        {
            if Y[6] != 7 {
                std::process::exit(5);
            }
        }

        {
            let hp = HoldP { p: Y.as_ptr() };
            if *hp.p != 1 {
                std::process::exit(6);
            }

            let ha = &*(Y.as_ptr() as *const HoldA);

            // Rust does not support directly accessing a flexible array member or offsetof
            // Rust has a well defined memory layout for structs so we skip this check

            if (*ha).a.as_ptr().offset(2).read() != 3 {
                std::process::exit(8);
            }
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }
    }
}