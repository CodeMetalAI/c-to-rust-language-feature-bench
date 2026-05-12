static mut G_SEED: i32 = 0;

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32) -> i32 {
    unsafe { G_SEED + x + 1000 }
}

fn v1(x: i32) -> i32 {
    unsafe { G_SEED + x - 2000 }
}

enum FunctionType {
    V0,
    V1,
}

fn fpfi(pf: fn(i64) -> i32, k: i32) -> FunctionType {
    let t = k as i64 * 97 + 1234;
    unsafe {
        G_SEED = pf(t) + k;
    }
    if unsafe { G_SEED & 1 } == 0 {
        FunctionType::V0
    } else {
        FunctionType::V1
    }
}

fn call_through(pf_type: FunctionType, x: i32) -> i32 {
    let r1 = match pf_type {
        FunctionType::V0 => v0(x),
        FunctionType::V1 => v1(x),
    };
    let r2 = r1; // simulating variadic function behavior
    let r3 = r1; // simulating variadic function behavior
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let mut vf = fpfi(cb0, 17);
    {
        let expected_seed = cb0(17 * 97 + 1234) + 17;
        let base = expected_seed + 40;
        match vf {
            FunctionType::V0 => {
                if v0(40) != base + 1000 { return; }
                if v0(40) != base + 1000 { return; }
                if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) { return; }
            },
            FunctionType::V1 => {
                if v1(40) != base - 2000 { return; }
                if v1(40) != base - 2000 { return; }
                if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) { return; }
            },
        }
    }

    vf = fpfi(cb1, 8);
    {
        let expected_seed = cb1(8 * 97 + 1234) + 8;
        let base = expected_seed + 9;
        match vf {
            FunctionType::V0 => {
                if v0(9) != base + 1000 { return; }
                if v0(9) != base + 1000 { return; }
                if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) { return; }
            },
            FunctionType::V1 => {
                if v1(9) != base - 2000 { return; }
                if v1(9) != base - 2000 { return; }
                if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) { return; }
            },
        }
    }
}