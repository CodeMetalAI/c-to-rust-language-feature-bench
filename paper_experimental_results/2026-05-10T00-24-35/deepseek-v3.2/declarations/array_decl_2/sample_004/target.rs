use std::mem;

type usize = u64;

static mut X: *const i32 = std::ptr::null();
static mut Y: [i32; 7] = [0; 7];

static mut BACKING: [i32; 3] = [10, 20, 30];

#[repr(C)]
struct HoldP {
    p: *const i32,
}

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32; 0],
}

const fn offset_of<T, U>(_: &T, member: *const U) -> usize {
    member as usize
}

unsafe fn sum_ptr(p: *const i32, n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += *p.add(i as usize);
        i += 1;
    }
    s
}

unsafe fn sum_arr7(a: *const i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < 7 {
        s += *a.add(i as usize);
        i += 1;
    }
    s
}

unsafe fn mutate_through_pointer(p: *mut i32) -> i32 {
    *p.add(1) += 5;
    *p.add(0) + *p.add(1) + *p.add(2)
}

fn main() {
    unsafe {
        X = BACKING.as_ptr();
        
        Y[0] = 1;
        Y[1] = 2;
        Y[2] = 3;
        Y[3] = 4;
        Y[4] = 5;
        Y[5] = 6;
        Y[6] = 7;

        if sum_arr7(Y.as_ptr()) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            std::process::exit(1);
        }

        if sum_ptr(X, 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        if mutate_through_pointer(BACKING.as_mut_ptr()) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if BACKING[1] != 25 {
            std::process::exit(4);
        }

        {
            let t = Y.as_ptr();
            if *t.add(6) != 7 {
                std::process::exit(5);
            }
        }

        {
            let mut hp = HoldP { p: std::ptr::null() };
            hp.p = Y.as_ptr();
            if *hp.p != 1 {
                std::process::exit(6);
            }

            let ha = Y.as_ptr() as *const HoldA;
            
            let a_ptr = (*ha).a.as_ptr();
            let ha_ptr = ha as *const u8;
            let expected_offset = offset_of(&*ha, a_ptr);
            
            if a_ptr as *const u8 != ha_ptr.add(expected_offset) {
                std::process::exit(7);
            }

            if *a_ptr.add(2) != 3 {
                std::process::exit(8);
            }
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }

        std::process::exit(0);
    }
}