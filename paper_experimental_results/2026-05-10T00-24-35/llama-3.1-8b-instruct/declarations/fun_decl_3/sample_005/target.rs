static mut G_SEED: i32 = 0;

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579 as i64) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3 as i64 - 11) as i32
}

fn v0(x: i32, _: i32, _: i32, _: i32, _: i32) -> i32 {
    G_SEED + x + 1000
}

fn v1(x: i32, _: i32, _: i32, _: i32, _: i32) -> i32 {
    G_SEED + x - 2000
}

fn fpfi(pf: impl Fn(i64) -> i32, k: i32) -> impl Fn(i32, i32, i32, i32, i32) -> i32 {
    let mut t = k * 97 + 1234;
    G_SEED = pf(t) + k;
    if G_SEED & 1 == 0 {
        Some(v0)
    } else {
        Some(v1)
    }
}

fn call_through(pf: impl Fn(i32, i32, i32, i32, i32) -> i32, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x, 1, 2, 3, 4);
    let r3 = pf(x, 1.25, &pf, 0x1122334455667788 as i64);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let mut vf = None;

    vf = Some(fpfi(cb0, 17));

    {
        let expected_seed = cb0(17 * 97 + 1234) + 17;
        let base = expected_seed + 40;

        match vf {
            Some(v0) if v0(40) != base + 1000 => panic!("expected 3000"),
            Some(v0) if v0(40, 1, 2, 3, 4) != base + 1000 => panic!("expected 3000"),
            Some(v0) if call_through(v0, 40) != (base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9) => panic!("expected 3000"),
            Some(v1) if v1(40) != base - 2000 => panic!("expected -2000"),
            Some(v1) if v1(40, 1, 2, 3, 4) != base - 2000 => panic!("expected -2000"),
            Some(v1) if call_through(v1, 40) != (base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9) => panic!("expected -2000"),
            None => panic!("expected Some(_)")
        }
    }

    vf = Some(fpfi(cb1, 8));

    {
        let expected_seed = cb1(8 * 97 + 1234) + 8;
        let base = expected_seed + 9;

        match vf {
            Some(v0) if v0(9) != base + 1000 => panic!("expected 3000"),
            Some(v0) if v0(9, 0, 0, 0, 0) != base + 1000 => panic!("expected 3000"),
            Some(v0) if call_through(v0, 9) != (base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9) => panic!("expected 3000"),
            Some(v1) if v1(9) != base - 2000 => panic!("expected -2000"),
            Some(v1) if v1(9, 0, 0, 0, 0) != base - 2000 => panic!("expected -2000"),
            Some(v1) if call_through(v1, 9) != (base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9) => panic!("expected -2000"),
            None => panic!("expected Some(_)")
        }
    }

    assert!(true);
}