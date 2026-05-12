type Usize = usize;

static mut Y: [i32; 7] = [0; 7];
static mut BACKING: [i32; 3] = [10, 20, 30];

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

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
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

        if sum_ptr(&BACKING, 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        if mutate_through_pointer(&mut BACKING) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if BACKING[1] != 25 {
            std::process::exit(4);
        }

        {
            let t = &Y;
            if t[6] != 7 {
                std::process::exit(5);
            }
        }

        {
            struct HoldP<'a> {
                p: &'a [i32],
            }

            let hp = HoldP { p: &Y };
            if hp.p[0] != 1 {
                std::process::exit(6);
            }

            let y_ptr = Y.as_ptr() as *const u8;
            let a_offset = std::mem::size_of::<i32>();
            let a_ptr = unsafe { y_ptr.add(a_offset) };
            
            if a_ptr != y_ptr.wrapping_add(a_offset) {
                std::process::exit(7);
            }

            if Y[2] != 3 {
                std::process::exit(8);
            }
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}