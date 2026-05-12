fn hmix(a: i32, b: i32) -> i32 {
    let mut ua = a as u32;
    let mut ub = b as u32;
    ua ^= ua >> 16;
    ub ^= ub >> 15;
    (ua.wrapping_mul(1103515245) + ub.wrapping_mul(12345)) as i32
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
    let mut v = [40, 43, 46, 49, 52, 55];
    let sel = hmix(v[0], v[5]) & 3;
    let sel = if sel == 3 { 2 } else { sel };

    let (before_x, before_y) = (v[sel], v[sel + 1]);
    let out = run(match sel {
        0 => f0,
        1 => f1,
        _ => f2,
    }, &mut v[sel], &mut v[sel + 1]);

    if sel == 0 {
        if v[sel] != before_y || v[sel + 1] != before_x {
            std::process::exit(1 + (v[sel] != before_y) as i32);
        }
        if (out ^ 3) != (hmix(v[sel], v[sel + 1]) ^ (hmix(v[sel], v[sel + 1]) + 3)) {
            std::process::exit(3);
        }
    } else if sel == 1 {
        if v[sel] != before_x + 7 || v[sel + 1] != before_y - 3 {
            std::process::exit(4 + (v[sel] != before_x + 7) as i32);
        }
    } else {
        let d = if before_x > before_y { before_x - before_y } else { before_y - before_x };
        if v[sel] != d || v[sel + 1] != before_x + before_y {
            std::process::exit(6 + (v[sel] != d) as i32);
        }
    }
}