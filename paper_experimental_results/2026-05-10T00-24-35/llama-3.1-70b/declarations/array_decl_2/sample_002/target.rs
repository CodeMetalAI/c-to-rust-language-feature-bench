fn main() {
    static mut Y: [i32; 7] = [0; 7];
    static mut BACKING: [i32; 3] = [10, 20, 30];
    let mut x = &mut BACKING as *mut _;

    unsafe {
        Y[0] = 1;
        Y[1] = 2;
        Y[2] = 3;
        Y[3] = 4;
        Y[4] = 5;
        Y[5] = 6;
        Y[6] = 7;
    }

    if sum_arr7(&Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        return;
    }

    if sum_ptr(x, 3) != (10 + 20 + 30) {
        return;
    }

    if mutate_through_pointer(x) != (10 + 25 + 30) {
        return;
    }

    if unsafe { *x.offset(1) } != 25 {
        return;
    }

    {
        let t = &mut Y as *mut _;
        if unsafe { *t.offset(6) } != 7 {
            return;
        }
    }

    {
        struct HoldP {
            p: *mut i32,
        }
        struct HoldA {
            tag: i32,
            a: [i32],
        }

        let hp = HoldP { p: &mut Y as *mut _ };
        if unsafe { *hp.p } != 1 {
            return;
        }

        let ha = &mut Y as *mut _ as *mut HoldA;
        if std::ptr::addr_of!((*ha).a[0]) as *mut u8
            != std::ptr::addr_of!(*ha) as *mut u8
                + (std::mem::size_of::<HoldA>() - std::mem::size_of::<i32>())
        {
            return;
        }

        if unsafe { (*ha).a[2] } != 3 {
            return;
        }
    }

    if unsafe { Y[0] } != 1 {
        return;
    }
}

fn sum_ptr(p: *mut i32, n: i32) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += unsafe { *p.offset(i as isize) };
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    for i in 0..7 {
        s += a[i];
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.offset(1) += 5;
    }
    unsafe { *p } + unsafe { *p.offset(1) } + unsafe { *p.offset(2) }
}