#[repr(C)]
struct HoldP {
    p: *const i32,
}

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32; 7],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += unsafe { *p.offset(i as isize) };
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32]) -> i32 {
    let mut s = 0;
    for &x in a {
        s += x;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.offset(1) += 5;
    }
    let mut sum = 0;
    for i in 0..3 {
        sum += unsafe { *p.offset(i as isize) };
    }
    sum
}

fn main() {
    let mut y = [1, 2, 3, 4, 5, 6, 7];
    let backing = [10, 20, 30];
    let mut x = backing.as_mut_ptr();

    if sum_arr7(&y) != 28 {
        std::process::exit(1);
    }

    if sum_ptr(x, 3) != 60 {
        std::process::exit(2);
    }

    if mutate_through_pointer(x) != 65 {
        std::process::exit(3);
    }

    if backing[1] != 25 != 25 {
        std::process::exit(4);
    }

    {
        let t = y.as_ptr();
        if unsafe { *t.offset(6) } != 7 {
            std::process::exit(5);
        }
    }

    {
        let mut hp = HoldP {
            p: y.as_ptr(),
        };
        if unsafe { *hp.p } != 1 {
            std::process::exit(6);
        }

        let ha = y.as_ptr() as *const HoldA;
        if (ha as *const u8).offset(4) != ha as *const _ as *const u8 {
            std::process::exit(7);
        }

        if unsafe { (*ha).a[2] } != 3 {
            std::process::exit(8);
        }
    }

    if y[0] != 1 {
        std::process::exit(9);
    }
}