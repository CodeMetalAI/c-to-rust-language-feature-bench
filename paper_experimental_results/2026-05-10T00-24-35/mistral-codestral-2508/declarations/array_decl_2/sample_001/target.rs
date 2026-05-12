struct HoldP {
    p: *mut i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}

static mut Y: [i32; 7] = [0; 7];

static mut BACKING: [i32; 3] = [10, 20, 30];
static mut X: *mut i32 = unsafe { &mut BACKING[0] };

fn sum_ptr(p: *const i32, n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        unsafe {
            s += *p.offset(i as isize);
        }
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
    }
    unsafe { *p.offset(0) + *p.offset(1) + *p.offset(2) }
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
    }

    if sum_arr7(unsafe { &Y }) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        std::process::exit(1);
    }

    if sum_ptr(X, 3) != (10 + 20 + 30) {
        std::process::exit(2);
    }

    if mutate_through_pointer(X) != (10 + 25 + 30) {
        std::process::exit(3);
    }

    if unsafe { BACKING[1] } != 25 {
        std::process::exit(4);
    }

    {
        let t = unsafe { &Y };
        if t[6] != 7 {
            std::process::exit(5);
        }
    }

    {
        let mut hp = HoldP { p: unsafe { &mut Y[0] } };
        if unsafe { *hp.p } != 1 {
            std::process::exit(6);
        }

        let ha = unsafe { &*(Y.as_ptr() as *const HoldA) };

        if unsafe { (ha.a.as_ptr() as *const u8).offset(0) } != unsafe { (ha as *const HoldA as *const u8).offset(std::mem::size_of::<i32>() as isize) } {
            std::process::exit(7);
        }

        if unsafe { *ha.a.as_ptr().offset(2) } != 3 {
            std::process::exit(8);
        }
    }

    if unsafe { Y[0] } != 1 {
        std::process::exit(9);
    }
}