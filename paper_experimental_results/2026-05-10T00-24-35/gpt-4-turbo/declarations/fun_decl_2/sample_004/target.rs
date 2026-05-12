fn hmix(a: i32, b: i32) -> i32 {
    let ua = a as u32;
    let ub = b as u32;
    let ua = ua ^ (ua >> 16);
    let ub = ub ^ (ub >> 15);
    ((ua * 1103515245 + ub * 12345) as i32)
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
    let mut v: Vec<i32> = (0..6).map(|i| 40 + i * 3).collect();
    let sel = (hmix(v[0], v[5]) & 3) as usize;
    let sel = if sel == 3 { 2 } else { sel };

    let (x, y) = (v[sel], v[sel + 1]);

    let out = {
        let mut x = x;
        let mut y = y;
        let fns: Vec<&dyn Fn(&mut i32, &mut i32) -> i32> = vec![&f0, &f1, &f2];
        let out = run(fns[sel], &mut x, &mut y);

        if sel == 0 {
            if x != y || y != x {
                std::process::exit(1);
            }
            if (out ^ 3) != (hmix(x, y) ^ (hmix(x, y) + 3)) {
                std::process::exit(3);
            }
        } else if sel == 1 {
            if x != v[sel] + 7 {
                std::process::exit(4);
            }
            if y != v[sel + 1] - 3 {
                std::process::exit(5);
            }
        } else {
            let d = (v[sel] - v[sel + 1]).abs();
            if x != d {
                std::process::exit(6);
            }
            if y != v[sel] + v[sel + 1] {
                std::process::exit(7);
            }
        }
        
        out
    };

    println!("{}", out);
}