struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}

static BACKING: [i32; 3] = [10, 20, 30];
static mut X: *const i32 = BACKING.as_ptr();

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        unsafe {
            s += *p.add(i);
        }
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    for &value in a.iter() {
        s += value;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.add(1) += 5;
        *p + *p.add(1) + *p.add(2)
    }
}

fn main() -> i32 {
    let mut y = [0; 7];
    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        return 1;
    }

    unsafe {
        if sum_ptr(X, 3) != (10 + 20 + 30) {
            return 2;
        }

        if mutate_through_pointer(X as *mut i32) != (10 + 25 + 30) {
            return 3;
        }

        if *X.add(1) != 25 {
            return 4;
        }
    }

    {
        let t = &y as *const _ as *const i32;
        unsafe {
            if *t.add(6) != 7 {
                return 5;
            }
        }
    }

    {
        let mut hp = HoldP { p: y.as_ptr() };
        if unsafe { *hp.p } != 1 {
            return 6;
        }

        let ha = &y as *const _ as *const HoldA;

        if (ha as *const u8) != (ha as *const u8).wrapping_add(std::mem::size_of::<HoldA>() as isize) {
            return 7;
        }

        if unsafe { *ha.as_ref().a.get_unchecked(2) } != 3 {
            return 8;
        }
    }

    if y[0] != 1 {
        return 9;
    }

    0
}