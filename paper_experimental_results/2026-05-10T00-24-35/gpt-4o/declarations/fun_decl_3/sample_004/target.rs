static mut G_SEED: i32 = 0;

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32, _args: &[i32]) -> i32 {
    unsafe { G_SEED + x + 1000 }
}

fn v1(x: i32, _args: &[i32]) -> i32 {
    unsafe { G_SEED + x - 2000 }
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> fn(i32, &[i32]) -> i32 {
    let t = (k as i64) * 97 + 1234;
    unsafe {
        G_SEED = pf(t) + k;
        if (G_SEED & 1) == 0 {
            v0
        } else {
            v1
        }
    }
}

fn call_through(pf: fn(i32, &[i32]) -> i32, x: i32) -> i32 {
    let r1 = pf(x, &[]);
    let r2 = pf(x, &[1, 2, 3]);
    let r3 = pf(x, &[1, 2, 3]); // Simulating call with different types
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() -> i32 {
    let mut vf: fn(i32, &[i32]) -> i32;

    vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17 as i64) * 97 + 1234) + 17;
        let base = expected_seed + 40;

        if vf as usize == v0 as usize {
            if vf(40, &[]) != base + 1000 {
                return 1;
            }
            if vf(40, &[1, 2]) != base + 1000 {
                return 2;
            }
            if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 3;
            }
        } else {
            if vf(40, &[]) != base - 2000 {
                return 4;
            }
            if vf(40, &[1, 2]) != base - 2000 {
                return 5;
            }
            if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 6;
            }
        }
    }

    vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8 as i64) * 97 + 1234) + 8;
        let base = expected_seed + 9;

        if vf as usize == v0 as usize {
            if vf(9, &[]) != base + 1000 {
                return 7;
            }
            if vf(9, &[0, 0, 0, 0]) != base + 1000 {
                return 8;
            }
            if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 9;
            }
        } else {
            if vf(9, &[]) != base - 2000 {
                return 10;
            }
            if vf(9, &[0, 0, 0, 0]) != base - 2000 {
                return 11;
            }
            if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 12;
            }
        }
    }

    0
}