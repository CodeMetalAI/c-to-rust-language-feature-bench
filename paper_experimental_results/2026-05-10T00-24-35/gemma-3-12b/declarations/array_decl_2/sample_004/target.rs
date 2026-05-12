typedef unsigned long usize;

extern "C" {
    static mut x: *mut i32;
    static mut y: [i32; 7];
    static BACKING: [i32; 3];
}

#[macro_use]
extern "C" {
    fn sum_ptr(p: *const i32, n: i32) -> i32;
    fn sum_arr7(a: *const i32) -> i32;
    fn mutate_through_pointer(p: *mut i32) -> i32;
}

fn main() {
    unsafe {
        y[0] = 1;
        y[1] = 2;
        y[2] = 3;
        y[3] = 4;
        y[4] = 5;
        y[5] = 6;
        y[6] = 7;

        x = &BACKING[0];

        if sum_arr7(y.as_ptr()) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            return 1;
        }

        if sum_ptr(x, 3) != (10 + 20 + 30) {
            return 2;
        }

        if mutate_through_pointer(x) != (10 + 25 + 30) {
            return 3;
        }

        if BACKING[1] != 25 {
            return 4;
        }

        {
            let t = y.as_ptr();
            if *t.add(6) != 7 {
                return 5;
            }
        }

        {
            struct HoldP {
                p: *mut i32,
            }
            struct HoldA {
                tag: i32,
                a: [i32; 7],
            }

            let mut hp = HoldP { p: y.as_mut_ptr() };
            if *hp.p.add(0) != 1 {
                return 6;
            }

            let ha = &*(y.as_ptr() as *mut HoldA);

            if (y.as_ptr() as usize) + std::mem::size_of::<i32>() != (ha as *const HoldA as usize) + std::mem::size_of::<i32>() {
                return 7;
            }

            if ha.a[2] != 3 {
                return 8;
            }
        }

        if y[0] != 1 {
            return 9;
        }
    }

    0
}