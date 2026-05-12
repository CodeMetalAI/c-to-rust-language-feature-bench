fn hmix(a: i32, b: i32) -> i32 {
    let ua = a as u32;
    let ub = b as u32;
    let ua = ua ^ (ua >> 16);
    let ub = ub ^ (ub >> 15);
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

    let mut v: [i32; 6] = [40, 43, 46, 49, 52, 55];

    let mut sel = (hmix(v[0], v[5]) & 3) as usize;
    if sel == 3 {
        sel = 2;
    }

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

    // In Rust, function pointers are not nullable, so this check is always false
    // if apfi[sel] == (fn(&mut i32, &mut i32) -> i32) { // This would not compile
    //     std::process::exit(8);
    // }

    std::process::exit(0);
}