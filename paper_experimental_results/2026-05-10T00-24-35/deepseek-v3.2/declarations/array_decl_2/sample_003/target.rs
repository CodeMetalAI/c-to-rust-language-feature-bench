use std::mem;

type Usize = usize;

static mut X: *const i32 = std::ptr::null();
static mut Y: [i32; 7] = [0; 7];

static mut BACKING: [i32; 3] = [10, 20, 30];

struct HoldP {
    p: *const i32,
}

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32; 0],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        unsafe {
            s += *p.add(i);
        }
        i += 1;
    }
    s
}

fn sum_arr7(a: [i32; 7]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < 7 {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.add(1) += 5;
        *p.add(0) + *p.add(1) + *p.add(2)
    }
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

        if sum_arr7(Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
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
            if unsafe { *t.add(6) } != 7 {
                std::process::exit(5);
            }
        }

        {
            let mut hp = HoldP {
                p: std::ptr::null(),
            };
            hp.p = Y.as_ptr();
            if unsafe { *hp.p } != 1 {
                std::process::exit(6);
            }

            let ha = Y.as_ptr() as *const HoldA;

            let offset_of_a = mem::offset_of!(HoldA, a);
            
            let a_ptr = unsafe { &(*ha).a } as *const [i32; 0] as *const u8;
            let ha_ptr = ha as *const u8;
            
            if a_ptr != unsafe { ha_ptr.add(offset_of_a) } {
                std::process::exit(7);
            }

            if unsafe { *(*ha).a.as_ptr().add(2) } != 3 {
                std::process::exit(8);
            }
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }
    }
    
    std::process::exit(0);
}