fn hmix(a: i32, b: i32) -> i32 {
    let mut ua = a as u32;
    let mut ub = b as u32;
    ua ^= ua >> 16;
    ub ^= ub >> 15;
    (ua * 1103515245u + ub * 12345u) as i32
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
    let d = if d < 0 { -d } else { d };
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

    let mut v = [0; 6];
    for i in 0..6 {
        v[i] = 40 + i * 3;
    }

    let sel = (hmix(v[0], v[5]) & 3) as usize;
    let sel = if sel == 3 { 2 } else { sel };

    let mut x = &mut v[sel];
    let mut y = &mut v[sel + 1];

    let before_x = *x;
    let before_y = *y;

    let out = run(apfi[sel], x, y);

    if sel == 0 {
        if *x != before_y {
            std::process::exit(1);
        }
        if *y != before_x {
            std::process::exit(2);
        }
        if (out ^ 3) != (hmix(*x, *y) ^ (hmix(*x, *y) + 3)) {
            std::process::exit(3);
        }
    } else if sel == 1 {
        if *x != before_x + 7 {
            std::process::exit(4);
        }
        if *y != before_y - 3 {
            std::process::exit(5);
        }
    } else {
        let d = before_x - before_y;
        let d = if d < 0 { -d } else { d };
        if *x != d {
            std::process::exit(6);
        }
        if *y != before_x + before_y {
            std::process::exit(7);
        }
    }

    if apfi[sel] == (fn(&mut i32, &mut i32) -> i32)::default() {
        std::process::exit(8);
    }

    std::process::exit(0);
}