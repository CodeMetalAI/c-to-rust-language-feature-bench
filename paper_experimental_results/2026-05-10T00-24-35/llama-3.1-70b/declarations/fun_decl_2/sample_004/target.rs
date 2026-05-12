fn hmix(a: i32, b: i32) -> i32 {
    let ua = a as u32;
    let ub = b as u32;
    ua ^ (ua >> 16) as i32 + (ub * 12345 + (ua * 1103515245u) as i32)
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
    let d = if a - b < 0 { -(a - b) } else { a - b };
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
    let mut apfi = [f0, f1, f2];

    let mut v = [0; 6];
    for (i, val) in v.iter_mut().enumerate() {
        *val = 40 + i * 3;
    }

    let sel = (hmix(v[0], v[5]) & 3) as usize;
    let sel = if sel == 3 { 2 } else { sel };

    let x = &mut v[sel];
    let y = &mut v[sel + 1];

    let before_x = *x;
    let before_y = *y;

    let out = run(apfi[sel], x, y);

    if sel == 0 {
        if *x!= before_y {
            std::process::exit(1);
        }
        if *y!= before_x {
            std::process::exit(2);
        }
        if (out ^ 3)!= (hmix(*x, *y) ^ (hmix(*x, *y) + 3)) {
            std::process::exit(3);
        }
    } else if sel == 1 {
        if *x!= before_x + 7 {
            std::process::exit(4);
        }
        if *y!= before_y - 3 {
            std::process::exit(5);
        }
    } else {
        let d = if before_x - before_y < 0 { -(before_x - before_y) } else { before_x - before_y };
        if *x!= d {
            std::process::exit(6);
        }
        if *y!= before_x + before_y {
            std::process::exit(7);
        }
    }

    if apfi[sel] == std::ptr::null::<fn(&mut i32, &mut i32) -> i32>() {
        std::process::exit(8);
    }
}