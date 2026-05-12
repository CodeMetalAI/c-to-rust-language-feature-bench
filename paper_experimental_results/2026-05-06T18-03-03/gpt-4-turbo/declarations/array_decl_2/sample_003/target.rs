fn main() {
    static mut Y: [i32; 7] = [0; 7];
    static mut BACKING: [i32; 3] = [10, 20, 30];
    static mut X: *const i32 = BACKING.as_ptr();

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

        if sum_ptr(X, 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        if mutate_through_pointer(&mut BACKING) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if BACKING[1] != 25 {
            std::process::exit(4);
        }

        let t = &Y;
        if t[6] != 7 {
            std::process::exit(5);
        }

        let hp = HoldP { p: &Y };
        if hp.p[0] != 1 {
            std::process::exit(6);
        }

        let ha = &Y as *const i32 as *const HoldA;
        if &(*ha).a as *const _ as usize != ha as usize + std::mem::size_of::<i32>() {
            std::process::exit(7);
        }

        if (*ha).a[2] != 3 {
            std::process::exit(8);
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }
    }
}

struct HoldP<'a> {
    p: &'a [i32],
}

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(p, n) };
    slice.iter().sum()
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    a.iter().sum()
}

fn mutate_through_pointer(p: &mut [i32; 3]) -> i32 {
    p[1] += 5;
    p.iter().sum()
}