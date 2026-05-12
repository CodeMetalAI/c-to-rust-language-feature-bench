fn hmix(a: i32, b: i32) -> i32 {
    let mut ua = a as u32;
    let mut ub = b as u32;
    ua ^= ua >> 16;
    ub ^= ub >> 15;
    (ua.wrapping_mul(1103515245u32).wrapping_add(ub.wrapping_mul(12345u32))) as i32
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
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let apfi: [fn(&mut i32, &mut i32) -> i32; 3] = [f0, f1, f2];

    let mut v = [0i32; 6];
    let mut i = 0;
    while i < 6 {
        v[i] = 40 + (i as i32) * 3;
        i += 1;
    }

    let mut sel = (hmix(v[0], v[5]) & 3) as usize;
    if sel == 3 {
        sel = 2;
    }

    let before_x = v[sel];
    let before_y = v[sel + 1];

    let out = {
        let (left, right) = v.split_at_mut(sel + 1);
        let x = &mut left[sel];
        let y = &mut right[0];
        run(apfi[sel], x, y)
    };

    if sel == 0 {
        if v[sel] != before_y {
            return 1;
        }
        if v[sel + 1] != before_x {
            return 2;
        }
        if (out ^ 3) != (hmix(v[sel], v[sel + 1]) ^ (hmix(v[sel], v[sel + 1]) + 3)) {
            return 3;
        }
    } else if sel == 1 {
        if v[sel] != before_x + 7 {
            return 4;
        }
        if v[sel + 1] != before_y - 3 {
            return 5;
        }
    } else {
        let mut d = before_x - before_y;
        if d < 0 {
            d = -d;
        }
        if v[sel] != d {
            return 6;
        }
        if v[sel + 1] != before_x + before_y {
            return 7;
        }
    }

    // The null check in C always passes since apfi[sel] is always a valid function pointer
    // In Rust, fn pointers cannot be null, so this check is not needed

    0
}