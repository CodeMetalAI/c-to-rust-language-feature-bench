fn hmix(a: i32, b: i32) -> i32 {
    let ua = a as u32;
    let ub = b as u32;
    ua ^= ua >> 16;
    ub ^= ub >> 15;
    ((ua * 1103515245u32 + ub * 12345u32) as i32)
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
    (hmix(*x, *y) ^ 1) as i32
}

fn f2(x: &mut i32, y: &mut i32) -> i32 {
    let a = *x;
    let b = *y;
    let d = if a - b < 0 { -(a - b) } else { a - b };
    *x = d;
    *y = a + b;
    (hmix(*x, *y) ^ 2) as i32
}

fn run(pf: fn(&mut i32, &mut i32) -> i32, a: &mut i32, b: &mut i32) -> i32 {
    let r1 = pf(a, b);
    let r2 = pf(a, b);
    (r1 ^ (r2 + 3)) as i32
}

fn main() {
    let apfi: [fn(&mut i32, &mut i32) -> i32; 3] = [
        f0,
        f1,
        f2,
    ];

    let mut v: [i32; 6] = [
        40,
        43,
        46,
        49,
        52,
        55,
    ];

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

    match sel {
        0 => {
            if *x!= before_y {
                return 1;
            }
            if *y!= before_x {
                return 2;
            }
            if (out ^ 3)!= (hmix(*x, *y) ^ (hmix(*x, *y) + 3)) {
                return 3;
            }
        }
        1 => {
            if *x!= before_x + 7 {
                return 4;
            }
            if *y!= before_y - 3 {
                return 5;
            }
        }
        _ => {
            let d = before_x - before_y;
            if d < 0 {
                d = -d;
            }
            if *x!= d {
                return 6;
            }
            if *y!= before_x + before_y {
                return 7;
            }
        }
    }

    if apfi[sel] == std::ptr::null_mut() {
        return 8;
    }
}