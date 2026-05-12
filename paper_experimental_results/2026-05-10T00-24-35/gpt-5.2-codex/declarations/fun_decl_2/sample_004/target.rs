use std::process::exit;

fn hmix(a: i32, b: i32) -> i32 {
    let mut ua = a as u32;
    let mut ub = b as u32;
    ua ^= ua >> 16;
    ub ^= ub >> 15;
    let res = ua
        .wrapping_mul(1103515245u32)
        .wrapping_add(ub.wrapping_mul(12345u32));
    res as i32
}

fn f0(x: &mut i32, y: &mut i32) -> i32 {
    let t = *x;
    *x = *y;
    *y = t;
    hmix(*x, *y)
}

fn f1(x: &mut i32, y: &mut i32) -> i32 {
    let a = *x;
    let b = *y;
    *x = a + 7;
    *y = b - 3;
    hmix(*x, *y) ^ 1
}

fn f2(x: &mut i32, y: &mut i32) -> i32 {
    let a = *x;
    let b = *y;
    let mut d = a - b;
    if d < 0 {
        d = -d;
    }
    *x = d;
    *y = a + b;
    hmix(*x, *y) ^ 2
}

fn run(pf: fn(&mut i32, &mut i32) -> i32, a: &mut i32, b: &mut i32) -> i32 {
    let r1 = pf(a, b);
    let r2 = pf(a, b);
    r1 ^ (r2 + 3)
}

fn main() {
    let apfi: [fn(&mut i32, &mut i32) -> i32; 3] = [f0, f1, f2];

    let mut v = [0i32; 6];
    let mut i = 0;
    while i < 6 {
        v[i] = 40 + (i as i32) * 3;
        i += 1;
    }

    let mut sel = hmix(v[0], v[5]) & 3;
    if sel == 3 {
        sel = 2;
    }
    let sel_usize = sel as usize;

    let before_x = v[sel_usize];
    let before_y = v[sel_usize + 1];

    let out = {
        let (left, right) = v.split_at_mut(sel_usize + 1);
        let x = &mut left[sel_usize];
        let y = &mut right[0];
        run(apfi[sel_usize], x, y)
    };

    if sel == 0 {
        if v[sel_usize] != before_y {
            exit(1);
        }
        if v[sel_usize + 1] != before_x {
            exit(2);
        }
        let hx = hmix(v[sel_usize], v[sel_usize + 1]);
        if (out ^ 3) != (hx ^ (hx + 3)) {
            exit(3);
        }
    } else if sel == 1 {
        if v[sel_usize] != before_x + 7 {
            exit(4);
        }
        if v[sel_usize + 1] != before_y - 3 {
            exit(5);
        }
    } else {
        let mut d = before_x - before_y;
        if d < 0 {
            d = -d;
        }
        if v[sel_usize] != d {
            exit(6);
        }
        if v[sel_usize + 1] != before_x + before_y {
            exit(7);
        }
    }

    if (apfi[sel_usize] as usize) == 0 {
        exit(8);
    }

    exit(0);
}