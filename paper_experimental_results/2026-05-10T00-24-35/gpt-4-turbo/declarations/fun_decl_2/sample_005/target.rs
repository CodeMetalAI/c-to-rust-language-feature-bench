fn hmix(a: i32, b: i32) -> i32 {
    let mut ua = a as u32;
    let mut ub = b as u32;
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

fn run(pf: &dyn Fn(&mut i32, &mut i32) -> i32, a: &mut i32, b: &mut i32) -> i32 {
    let r1 = pf(a, b);
    let r2 = pf(a, b);
    r1 ^ (r2 + 3)
}

fn main() {
    let mut v = [40, 43, 46, 49, 52, 55];
    let sel = (hmix(v[0], v[5]) & 3) as usize;
    let sel = if sel == 3 { 2 } else { sel };

    let (x, y) = (&mut v[sel], &mut v[sel + 1]);
    let (before_x, before_y) = (*x, *y);

    let funcs: [fn(&mut i32, &mut i32) -> i32; 3] = [f0, f1, f2];
    let pf = &funcs[sel];
    let out = run(pf, x, y);

    if sel == 0 {
        if *x != before_y || *y != before_x {
            println!("Values not swapped correctly");
            std::process::exit(sel + 1);
        }
        if (out ^ 3) != (hmix(*x, *y) ^ (hmix(*x, *y) + 3)) {
            println!("Output check failed for selection 0");
            std::process::exit(3);
        }
    } else if sel == 1 {
        if *x != before_x + 7 || *y != before_y - 3 {
            println!("Incorrect arithmetic adjustment for selection 1");
            std::process::exit(sel + 3);
        }
    } else {  // sel == 2
        let d = if before_x > before_y { before_x - before_y } else { before_y - before_x };
        if *x != d || *y != before_x + before_y {
            println!("Incorrect arithmetic calculation for selection 2");
            std::process::exit(sel + 4);
        }
    }
}