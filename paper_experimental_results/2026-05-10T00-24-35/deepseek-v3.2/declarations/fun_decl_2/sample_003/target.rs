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
    let apfi: [fn(&mut i32, &mut i32) -> i32; 3] = [f0, f1, f2];

    let mut v = [0; 6];
    for i in 0..6 {
        v[i] = 40 + i * 3;
    }

    let sel = (hmix(v[0], v[5]) & 3) as usize;
    let sel = if sel == 3 { 2 } else { sel };

    let (x_idx, y_idx) = (sel, sel + 1);
    let before_x = v[x_idx];
    let before_y = v[y_idx];

    let out = run(apfi[sel], &mut v[x_idx], &mut v[y_idx]);

    let result = if sel == 0 {
        if v[x_idx] != before_y {
            1
        } else if v[y_idx] != before_x {
            2
        } else {
            let tmp = hmix(v[x_idx], v[y_idx]);
            if (out ^ 3) != (tmp ^ (tmp + 3)) {
                3
            } else {
                0
            }
        }
    } else if sel == 1 {
        if v[x_idx] != before_x + 7 {
            4
        } else if v[y_idx] != before_y - 3 {
            5
        } else {
            0
        }
    } else {
        let mut d = before_x - before_y;
        if d < 0 {
            d = -d;
        }
        if v[x_idx] != d {
            6
        } else if v[y_idx] != before_x + before_y {
            7
        } else {
            0
        }
    };

    std::process::exit(result);
}