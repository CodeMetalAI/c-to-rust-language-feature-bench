fn main() {
    static mut G_SEED: i32 = 0;

    fn cb0(x: i64) -> i32 { (x ^ 0x13579) as i32 + 7 }
    fn cb1(x: i64) -> i32 { (x * 3) as i32 - 11 }

    fn v0(x: i32, _: i32) -> i32 { unsafe { G_SEED + x + 1000 } }
    fn v1(x: i32, _: i32) -> i32 { unsafe { G_SEED + x - 2000 } }

    fn fpfi(pf: fn(i64) -> i32, k: i32) -> fn(i32, i32) -> i32 {
        let t = k as i64 * 97 + 1234;
        unsafe {
            G_SEED = pf(t) + k;
            if (G_SEED & 1) == 0 { v0 } else { v1 }
        }
    }

    fn call_through(pf: fn(i32, i32) -> i32, x: i32) -> i32 {
        let r1 = pf(x, 0);
        let r2 = pf(x, 1);
        let r3 = pf(x, 2);
        r1 ^ (r2 + 5) ^ (r3 + 9)
    }

    // First test
    let mut vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0(17 * 97 + 1234) + 17;
        let base = expected_seed + 40;

        let return_code = if vf as usize == v0 as usize {
            if vf(40, 0) != base + 1000 { 1 }
            else if vf(40, 1) != base + 1000 { 2 }
            else if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) { 3 }
            else { 0 }
        } else {
            if vf(40, 0) != base - 2000 { 4 }
            else if vf(40, 1) != base - 2000 { 5 }
            else if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) { 6 }
            else { 0 }
        };
        if return_code != 0 {
            std::process::exit(return_code);
        }
    }

    // Second test
    vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1(8 * 97 + 1234) + 8;
        let base = expected_seed + 9;

        let return_code = if vf as usize == v0 as usize {
            if vf(9, 0) != base + 1000 { 7 }
            else if vf(9, 1) != base + 1000 { 8 }
            else if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) { 9 }
            else { 0 }
        } else {
            if vf(9, 0) != base - 2000 { 10 }
            else if vf(9, 1) != base - 2000 { 11 }
            else if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) { 12 }
            else { 0 }
        };
        if return_code != 0 {
            std::process::exit(return_code);
        }
    }
}