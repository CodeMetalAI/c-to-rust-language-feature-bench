fn main() {
    let mut g_seed = 0;

    fn cb0(x: i64) -> i32 { (x ^ 0x13579) as i32 + 7 }
    fn cb1(x: i64) -> i32 { (x * 3) as i32 - 11 }

    fn v0(g_seed: i32, x: i32) -> i32 { g_seed + x + 1000 }
    fn v1(g_seed: i32, x: i32) -> i32 { g_seed + x - 2000 }

    fn fpfi(pf: fn(i64) -> i32, k: i32, g_seed: &mut i32) -> fn(i32, i32) -> i32 {
        let t = k as i64 * 97 + 1234;
        *g_seed = pf(t) + k;
        if (*g_seed & 1) == 0 { v0 } else { v1 }
    }

    fn call_through(pf: fn(i32, i32) -> i32, g_seed: i32, x: i32) -> i32 {
        let r1 = pf(g_seed, x);
        let r2 = pf(g_seed, x);
        let r3 = pf(g_seed, x);
        r1 ^ (r2 + 5) ^ (r3 + 9)
    }

    let mut vf = fpfi(cb0, 17, &mut g_seed);
    let expected_seed = cb0(17 * 97 + 1234) + 17;
    let base = expected_seed + 40;

    if vf as usize == v0 as usize {
        if vf(base, 40) != base + 1000 { return println!("{}", 1); }
        if vf(base, 40) != base + 1000 { return println!("{}", 2); }
        if call_through(vf, base, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) { return println!("{}", 3); }
    } else {
        if vf(base, 40) != base - 2000 { return println!("{}", 4); }
        if vf(base, 40) != base - 2000 { return println!("{}", 5); }
        if call_through(vf, base, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) { return println!("{}", 6); }
    }

    vf = fpfi(cb1, 8, &mut g_seed);
    let expected_seed = cb1(8 * 97 + 1234) + 8;
    let base = expected_seed + 9;

    if vf as usize == v0 as usize {
        if vf(base, 9) != base + 1000 { return println!("{}", 7); }
        if vf(base, 9) != base + 1000 { return println!("{}", 8); }
        if call_through(vf, base, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) { return println!("{}", 9); }
    } else {
        if vf(base, 9) != base - 2000 { return println!("{}", 10); }
        if vf(base, 9) != base - 2000 { return println!("{}", 11); }
        if call_through(vf, base, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) { return println!("{}", 12); }
    }

    println!("0");
}