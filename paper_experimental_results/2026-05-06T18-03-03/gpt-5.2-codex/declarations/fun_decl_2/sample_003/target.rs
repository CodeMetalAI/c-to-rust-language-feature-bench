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
    *x = a.wrapping_add(7);
    *y = b.wrapping_sub(3);
    hmix(*x, *y) ^ 1
}

fn f2(x: &mut i32, y: &mut i32) -> i32 {
    let a = *x;
    let b = *y;
    let mut d = a.wrapping_sub(b);
    if d < 0 {
        d = d.wrapping_neg();
    }
    *x = d;
    *y = a.wrapping_add(b);
    hmix(*x, *y) ^ 2
}

fn run(pf: fn(&mut i32, &mut i32) -> i32, a: &mut i32, b: &mut i32) -> i32 {
    let r1 = pf(a, b);
    let r2 = pf(a, b);
    r1 ^ r2.wrapping_add(3)
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

    let (x, y) = {
        let (left, right) = v.split_at_mut(sel_usize + 1);
        let x = &mut left[sel_usize];
        let y = &mut right[0];
        (x, y)
    };

    let before_x = *x;
    let before_y = *y;

    let out = run(apfi[sel_usize], x, y);

    if sel == 0 {
        if *x != before_y {
            exit(1);
        }
        if *y != before_x {
            exit(2);
        }
        let h1 = hmix(*x, *y);
        let h2 = hmix(*x, *y).wrapping_add(3);
        if (out ^ 3) != (h1 ^ h2) {
            exit(3);
        }
    } else if sel == 1 {
        if *x != before_x.wrapping_add(7) {
            exit(4);
        }
        if *y != before_y.wrapping_sub(3) {
            exit(5);
        }
    } else {
        let mut d = before_x.wrapping_sub(before_y);
        if d < 0 {
            d = d.wrapping_neg();
        }
        if *x != d {
            exit(6);
        }
        if *y != before_x.wrapping_add(before_y) {
            exit(7);
        }
    }

    // Function pointers in Rust are never null, so this check is always false.

    exit(0);
}