static fn hmix(a: i32, b: i32) -> i32 {
    let ua = a as u32;
    let ub = b as u32;
    let mut ua_mut = ua ^ (ua >> 16);
    let mut ub_mut = ub ^ (ub >> 15);
    (ua_mut.wrapping_mul(1103515245) as i32).wrapping_add(ub_mut.wrapping_mul(12345) as i32)
}

static fn f0(x: *mut i32, y: *mut i32) -> i32 {
    let t = unsafe { *x };
    unsafe {
        *x = *y;
        *y = t;
    }
    hmix(*x, *y)
}

static fn f1(x: *mut i32, y: *mut i32) -> i32 {
    let a = unsafe { *x };
    let b = unsafe { *y };
    unsafe {
        *x = a + 7;
        *y = b - 3;
    }
    hmix(*x, *y) ^ 1
}

static fn f2(x: *mut i32, y: *mut i32) -> i32 {
    let a = unsafe { *x };
    let b = unsafe { *y };
    let d = a - b;
    let mut d_mut = if d < 0 { -d } else { d };
    unsafe {
        *x = d_mut;
        *y = a + b;
    }
    hmix(*x, *y) ^ 2
}

fn run(pf: fn(*mut i32, *mut i32) -> i32, a: *mut i32, b: *mut i32) -> i32 {
    let r1 = pf(a, b);
    let r2 = (pf)(a, b);
    r1 ^ (r2 + 3)
}

fn main() -> i32 {
    let apfi: [fn(*mut i32, *mut i32) -> i32; 3] = [f0, f1, f2];

    {
        let mut v = [0; 6];
        let mut i = 0;
        while i < 6 {
            v[i] = 40 + i * 3;
            i += 1;
        }

        let sel = (hmix(v[0], v[5]) & 3) as i32;
        let sel = if sel == 3 { 2 } else { sel };

        {
            let x = &mut v[sel as usize];
            let y = &mut v[sel as usize + 1];

            let before_x = unsafe { *x };
            let before_y = unsafe { *y };

            let out = run(apfi[sel as usize], x, y);

            if sel == 0 {
                if unsafe { *x } != before_y {
                    return 1;
                }
                if unsafe { *y } != before_x {
                    return 2;
                }
                if (out ^ 3) != (hmix(unsafe { *x }, unsafe { *y }) ^ (hmix(unsafe { *x }, unsafe { *y }) + 3)) {
                    return 3;
                }
            } else if sel == 1 {
                if unsafe { *x } != before_x + 7 {
                    return 4;
                }
                if unsafe { *y } != before_y - 3 {
                    return 5;
                }
            } else {
                let d = before_x - before_y;
                let mut d_mut = if d < 0 { -d } else { d };
                if unsafe { *x } != d_mut {
                    return 6;
                }
                if unsafe { *y } != before_x + before_y {
                    return 7;
                }
            }

            if apfi[sel as usize] == fn(*mut i32, *mut i32) -> i32 { 0 } {
                return 8;
            }
        }
    }

    0
}