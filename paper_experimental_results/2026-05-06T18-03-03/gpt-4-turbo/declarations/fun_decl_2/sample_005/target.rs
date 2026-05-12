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

fn run(pf: fn(&mut i32, &mut i32) -> i32, x: &mut i32, y: &mut i32) -> i32 {
    let r1 = pf(x, y);
    let r2 = pf(x, y);
    r1 ^ (r2 + 3)
}

fn main() {
    let mut v = [40, 43, 46, 49, 52, 55];
    let sel = (hmix(v[0], v[5]) & 3) as usize;
    let sel = if sel == 3 { 2 } else { sel };

    let (mut x, mut y) = (v[sel], v[sel + 1]);
    let (before_x, before_y) = (x, y);

    let out = match sel {
        0 => run(f0, &mut x, &mut y),
        1 => run(f1, &mut x, &mut y),
        2 => run(f2, &mut x, &mut y),
        _ => unreachable!(),
    };

    v[sel] = x;
    v[sel + 1] = y;

    if sel == 0 {
        if x != before_y || y != before_x || (out ^ 3) != (hmix(x, y) ^ (hmix(x, y) + 3)) {
            std::process::exit(1);
        }
    } else if sel == 1 {
        if x != before_x + 7 || y != before_y - 3 {
            std::process::exit(4);
        }
    } else {
        let d = if before_x > before_y { before_x - before_y } else { before_y - before_x };
        if x != d || y != before_x + before_y {
            std::process::exit(6);
        }
    }
}