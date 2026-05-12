fn hmix(a: i32, b: i32) -> i32 {
    let ua: u32 = (a as u32);
    let ub: u32 = (b as u32);
    ua ^= ua >> 16;
    ub ^= ub >> 15;
    (ua as i32) * 1103515245 + (ub as i32) * 12345
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
    let d = if a - b < 0 { -((a - b) as i32) } else { a - b };
    *x = d;
    *y = a + b;
    hmix(*x, *y) ^ 2
}

fn run(pf: fn(&mut i32, &mut i32) -> i32, a: &mut i32, b: &mut i32) -> i32 {
    let r1 = pf(a, b);
    let r2 = (pf)(a, b);
    r1 ^ (r2 + 3)
}

fn main() {
    let mut apfi: [fn(&mut i32, &mut i32) -> i32; 3] = [
        f0,
        f1,
        f2,
    ];

    let mut v: [i32; 6] = [40, 45, 50, 54, 57, 63];

    let sel = (hmix(v[0], v[5]) & 3) as usize;
    if sel == 2 {
        sel = 2;
    }

    let (x, y) = if sel <= 1 { (v.sel(), v.sel() + 1) } else { (v[sel], v[sel + 1]) };
    let x = &mut x;
    let y = &mut y;
    let (before_x, before_y) = (*x, *y);
    let out = run(apfi[sel], x, y);

    if sel == 0 {
        if *x != before_y {
            return 1;
        }
        if *y != before_x {
            return 2;
        }
        if (out ^ 3) != (hmix(*x, *y) ^ (hmix(*x, *y) + 3)) {
            return 3;
        }
    } else if sel == 1 {
        if *x != before_x + 7 {
            return 4;
        }
        if *y != before_y - 3 {
            return 5;
        }
    } else {
        let d = before_x - before_y;
        if d < 0 {
            d = -d;
        }
        if *x != d {
            return 6;
        }
        if *y != before_x + before_y {
            return 7;
        }
    }

    if apfi[sel] == std::ptr::null_mut() {
        return 8;
    }
}