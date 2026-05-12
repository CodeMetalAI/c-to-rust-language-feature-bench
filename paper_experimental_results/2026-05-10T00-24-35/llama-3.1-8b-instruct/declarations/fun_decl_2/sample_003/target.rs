fn hmix(a: i32, b: i32) -> i32 {
    let ua: u32 = a as u32;
    let ub: u32 = b as u32;
    ua ^= ua >> 16;
    ub ^= ub >> 15;
    (ua * 1103515245u32 + ub * 12345u32) as i32
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
    let d = a - b;
    if d < 0 {
        d = -d;
    }
    *x = d;
    *y = a + b;
    hmix(*x, *y) ^ 2
}

fn run(pf: impl FnMut(&mut i32, &mut i32) -> i32, a: &mut i32, b: &mut i32) -> i32 {
    let r1 = pf(a, b);
    let r2 = pf(a, b);
    r1 ^ (r2 + 3)
}

fn main() {
    let apfi = [f0, f1, f2];

    let mut v = [0; 6];
    for (i, v) in v.iter_mut().enumerate() {
        *v = 40 + i * 3;
    }

    let sel = (hmix(v[0], v[5]) & 3);
    if sel == 3 {
        sel = 2;
    }

    let mut x = &mut v[sel];
    let mut y = &mut v[sel + 1];

    let before_x = *x;
    let before_y = *y;

    let out = run(&mut apfi[sel], x, y);

    match sel {
        0 => {
            if *x!= before_y {
                panic!("expected x to be equal to y");
            }
            if *y!= before_x {
                panic!("expected y to be equal to x");
            }
            if (out ^ 3)!= (hmix(*x, *y) ^ (hmix(*x, *y) + 3)) {
                panic!("expected out to be equal to the result of hmix(x, y) ^ (hmix(x, y) + 3)");
            }
        }
        1 => {
            if *x!= before_x + 7 {
                panic!("expected x to be equal to x + 7");
            }
            if *y!= before_y - 3 {
                panic!("expected y to be equal to y - 3");
            }
        }
        _ => {
            let d = before_x - before_y;
            if d < 0 {
                d = -d;
            }
            if *x!= d {
                panic!("expected x to be equal to d");
            }
            if *y!= before_x + before_y {
                panic!("expected y to be equal to x + y");
            }
        }
    }

    panic!("expected apfi[sel] to be None");
}