static fn hmix(a: i32, b: i32) -> i32 {
    let ua = a as u32;
    let ub = b as u32;
    let mut ua_mut = ua ^ (ua >> 16);
    let mut ub_mut = ub ^ (ub >> 15);
    (ua_mut as i32 * 1103515245 + ub_mut as i32 * 12345)
}

static fn f0(x: &mut i32, y: &mut i32) -> i32 {
    let t = *x;
    *x = *y;
    *y = t;
    hmix(*x, *y)
}

static fn f1(x: &mut i32, y: &mut i32) -> i32 {
    let a = *x;
    let b = *y;
    *x = a + 7;
    *y = b - 3;
    hmix(*x, *y) ^ 1
}

static fn f2(x: &mut i32, y: &mut i32) -> i32 {
    let a = *x;
    let b = *y;
    let d = (a - b).abs();
    *x = d;
    *y = a + b;
    hmix(*x, *y) ^ 2
}

static fn run<F>(pf: &F, a: &mut i32, b: &mut i32) -> i32
where
    F: Fn(&mut i32, &mut i32) -> i32,
{
    let r1 = pf(a, b);
    let r2 = pf(a, b);
    r1 ^ (r2 + 3)
}

fn main() -> i32 {
    let apfi: [&dyn Fn(&mut i32, &mut i32) -> i32; 3] = [
        f0,
        f1,
        f2,
    ];

    {
        let mut v = [0; 6];
        let mut i = 0;
        while i < 6 {
            v[i] = 40 + i * 3;
            i += 1;
        }

        let sel = (hmix(v[0], v[5]) & 3) as i32;
        let sel = if sel == 3 { 2 } else { sel };

        {
            let x = &mut v[sel as usize];
            let y = &mut v[sel as usize + 1];

            let before_x = *x;
            let before_y = *y;

            let out = run(&apfi[sel as usize], x, y);

            if sel == 0 {
                if *x != before_y {
                    return 1;
                }
                if *y != before_x {
                    return 2;
                }
                if (out ^ 3) != (hmix(*x, *y) ^ (hmix(*x, *y) + 3)) {
                    return 3;
                }
            } else if sel == 1 {
                if *x != before_x + 7 {
                    return 4;
                }
                if *y != before_y - 3 {
                    return 5;
                }
            } else {
                let d = before_x - before_y;
                if d < 0 {
                    let d = -d;
                }
                if *x != d {
                    return 6;
                }
                if *y != before_x + before_y {
                    return 7;
                }
            }

            if apfi[sel as usize] as *const _ == std::ptr::null() {
                return 8;
            }
        }
    }

    0
}