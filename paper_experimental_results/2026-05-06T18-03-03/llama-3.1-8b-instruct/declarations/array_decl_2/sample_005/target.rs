// array_decl_2.rs
type Usize = u64;

extern crate core;

extern fn x() -> *mut i32;
extern fn y() -> &'static [i32];

static mut BACKING: [i32; 3] = [10, 20, 30];
static mut X: *mut i32 = BACKING.as_mut_ptr();

const OFFSET_OF: usize = std::mem::size_of::<i32>() as usize;

struct HoldP {
    p: *mut i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += *p.offset(i as isize);
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < a.len() {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe { p.offset(1).write_volatile(5 + *p.offset(1)); }
    *p.offset(0) + *p.offset(1) + *p.offset(2)
}

fn main() {
    unsafe {
        y()[0] = 1;
        y()[1] = 2;
        y()[2] = 3;
        y()[3] = 4;
        y()[4] = 5;
        y()[5] = 6;
        y()[6] = 7;

        if sum_arr7(&y()[..]) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            return 1;
        }

        if sum_ptr(X, 3) != (10 + 20 + 30) {
            return 2;
        }

        if mutate_through_pointer(X) != (10 + 25 + 30) {
            return 3;
        }

        if unsafe { BACKING[1] } != 25 {
            return 4;
        }

        {
            let t = &y()[..];
            if t[6] != 7 {
                return 5;
            }
        }

        {
            let hp = HoldP { p: y() };
            if hp.p[0] != 1 {
                return 6;
            }

            let ha = unsafe { &*(y() as *const HoldA) };
            if (ha as *const u8 as *const usize) != (ha as *const u8).offset(OFFSET_OF) {
                return 7;
            }

            if ha.a[2] != 3 {
                return 8;
            }
        }

        if y()[0] != 1 {
            return 9;
        }
    }

    0
}