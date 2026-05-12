use std::mem;

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}

static mut Y: [i32; 7] = [0; 7];
static mut BACKING: [i32; 3] = [10, 20, 30];
static mut X: *mut i32 = unsafe { &mut BACKING[0] };

const fn offset_of<T>(member: fn(&T) -> &i32) -> usize {
    let dummy = unsafe { mem::MaybeUninit::<T>::uninit().assume_init() };
    let member_ptr = member(&dummy) as *const i32;
    let struct_ptr = &dummy as *const T as *const i32;
    member_ptr as usize - struct_ptr as usize
}

fn sum_ptr(p: *const i32, n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += unsafe { *p.offset(i as isize) };
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

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.offset(1) += 5;
        *p.offset(0) + *p.offset(1) + *p.offset(2)
    }
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

        if sum_ptr(X, 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        if mutate_through_pointer(X) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if BACKING[1] != 25 {
            std::process::exit(4);
        }

        {
            let t = &Y as *const i32;
            if unsafe { *t.offset(6) } != 7 {
                std::process::exit(5);
            }
        }

        {
            let mut hp = HoldP { p: &Y as *const i32 };
            if unsafe { *hp.p.offset(0) } != 1 {
                std::process::exit(6);
            }

            let ha = &Y as *const i32 as *const HoldA;

            if unsafe { &ha.offset(0).a[0] as *const i32 as *const u8 } != unsafe { ha as *const u8 }.offset(offset_of(|ha: &HoldA| &ha.a[0]) as isize) {
                std::process::exit(7);
            }

            if unsafe { ha.offset(0).a[2] } != 3 {
                std::process::exit(8);
            }
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }
    }
}