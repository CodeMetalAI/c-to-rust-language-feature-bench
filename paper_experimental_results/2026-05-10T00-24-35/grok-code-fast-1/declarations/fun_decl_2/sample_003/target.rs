fn hmix(a: i32, b: i32) -> i32 {
    let ua = a as u32;
    let ub = b as u32;
    let ua = ua ^ (ua >> 16);
    let ub = ub ^ (ub >> 15);
    ((ua.wrapping_mul(1103515245u32) + ub.wrapping_mul(12345u32)) as u32) as i32
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

    let mut v: [i32; 6] = [0; 6];
    let mut i = 0;
    while i < 6 {
        v[i] = 40 + i * 3;
        i += 1;
    }

    let mut sel = (hmix(v[0], v[5]) as u32 & 3) as i32;
    if sel == 3 {
        sel = 2;
    }

    let sel_usize = sel as usize;
    let slice = &mut v[sel_usize..=sel_usize + 1];
    let x = &mut slice[0];
    let y = &mut slice[1];

    let before_x = *x;
    let before_y = *y;

    let out = run(apfi[sel_usize], x, y);

    if sel == 0 {
        if *x != before_y {
            std::process::exit(1);
        }
        if *y != before_x {
            std::process::exit(2);
        }
        let h = hmix(*x, *y);
        if (out ^ 3) != (h ^ (h + 3)) {
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

    // The check for apfi[sel] == 0 is omitted because function pointers in Rust are not nullable,
    // and in this code, it is never null, so it would never return 8.

    std::process::exit(0);
}