fn hmix(a: i32, b: i32) -> i32 {
    let ua = a as u32;
    let ub = b as u32;
    ua ^= ua >> 16;
    ub ^= ub >> 15;
    (ua * 1103515245 + ub * 12345) as i32
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
    let d = if a < b { a - b } else { b - a };
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
    let mut apfi = [f0, f1, f2];

    let mut v = [40 + i * 3; 6];
    let mut i = 0;
    while i < 6 {
        v[i] = 40 + i * 3;
        i += 1;
    }

    let sel = (hmix(v[0], v[5]) & 3) as usize;
    if sel == 3 {
        sel = 2;
    }

    let mut x = &mut v[sel];
    let mut y = &mut v[sel + 1];

    let before_x = *x;
    let before_y = *y;

    let out = run(apfi[sel], x, y);

    if sel == 0 {
        if *x!= before_y {
            return 1;
        }
        if *y!= before_x {
            return 2;
        }
        if (out ^ 3)!= (hmix(*x, *y) ^ (hmix(*x, *y) + 3)) {
            return 3;
        }
    } else if sel == 1 {
        if *x!= before_x + 7 {
            return 4;
        }
        if *y!= before_y - 3 {
            return 5;
        }
    } else {
        let d = if before_x < before_y { before_x - before_y } else { before_y - before_x };
        if *x!= d {
            return 6;
        }
        if *y!= before_x + before_y {
            return 7;
        }
    }

    if apfi[sel] == std::ptr::null_mut() {
        return 8;
    }
}