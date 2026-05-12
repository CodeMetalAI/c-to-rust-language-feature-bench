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

    let x = &mut v[sel];
    let y = &mut v[sel + 1];

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
        let tmp = hmix(*x, *y);
        if (out ^ 3) != (tmp ^ (tmp + 3)) {
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
        let mut d = before_x - before_y;
        if d < 0 {
            d = -d;
        }
        if *x != d {
            std::process::exit(6);
        }
        if *y != before_x + before_y {
            std::process::exit(7);
        }
    }

    // The C code checks if the function pointer is null, but in Rust
    // function pointers are never null, so this condition is always false.
    // We'll keep the logic but it will never trigger.
    if apfi[sel] as usize == 0 {
        std::process::exit(8);
    }

    std::process::exit(0);
}