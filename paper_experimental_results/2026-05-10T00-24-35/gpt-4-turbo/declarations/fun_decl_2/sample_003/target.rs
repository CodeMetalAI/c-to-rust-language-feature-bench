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

fn run<F>(pf: F, a: &mut i32, b: &mut i32) -> i32
where
    F: Fn(&mut i32, &mut i32) -> i32,
{
    let r1 = pf(a, b);
    let r2 = pf(a, b);
    r1 ^ (r2 + 3)
}

fn main() {
    let mut apfi = [f0, f1, f2];
    let mut v = [40, 43, 46, 49, 52, 55];

    let sel = (hmix(v[0], v[5]) & 3) as usize;
    let sel = if sel == 3 { 2 } else { sel };

    let before_x = v[sel];
    let before_y = v[sel + 1];

    let out = run(apfi[sel], &mut v[sel], &mut v[sel + 1]);
    let x = v[sel];
    let y = v[sel + 1];

    match sel {
        0 => {
            if x != before_y {
                std::process::exit(1);
            }
            if y != before_x {
                std::process::exit(2);
            }
            if (out ^ 3) != (hmix(x, y) ^ (hmix(x, y) + 3)) {
                std::process::exit(3);
            }
        }
        1 => {
            if x != before_x + 7 {
                std::process::exit(4);
            }
            if y != before_y - 3 {
                std::process::exit(5);
            }
        }
        2 => {
            let d = (before_x - before_y).abs();
            if x != d || y != before_x + before_y {
                std::process::exit(6);
            }
        }
        _ => {}
    }
}