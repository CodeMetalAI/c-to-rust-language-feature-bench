typedef unsigned long usize;

extern "C" {
    static mut x: *mut i32;
    static mut y: [i32; 7];
    static backing: [i32; 3];
}

#[macro_use]
extern "C" {
    fn sum_ptr(p: *const i32, n: i32) -> i32;
    fn sum_arr7(a: *const i32) -> i32;
    fn mutate_through_pointer(p: *mut i32) -> i32;
}

fn main() -> i32 {
    unsafe {
        y[0] = 1;
        y[1] = 2;
        y[2] = 3;
        y[3] = 4;
        y[4] = 5;
        y[5] = 6;
        y[6] = 7;

        if sum_arr7(&y[0]) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            return 1;
        }

        if sum_ptr(&backing[0], 3) != (10 + 20 + 30) {
            return 2;
        }

        if mutate_through_pointer(&mut backing[0]) != (10 + 25 + 30) {
            return 3;
        }

        if backing[1] != 25 {
            return 4;
        }

        {
            let t = &y[0];
            if t[6] != 7 {
                return 5;
            }
        }

        {
            let hp = std::ptr::null_mut();
            let ha = &y[0] as *const i32 as *mut i32;

            if (*hp).is_null() {
                panic!("hp is null");
            }


            if (*ha).is_null() {
                panic!("ha is null");
            }

            if (*ha)[0] != 1 {
                return 6;
            }


            let offset = std::mem::size_of::<i32>();
            if (*ha as *const u8) != ((ha as *const i32 as *const u8) + offset) {
                return 7;
            }
            if (*ha)[2] != 3 {
                return 8;
            }
        }

        if y[0] != 1 {
            return 9;
        }

        return 0;
    }
}