static mut g_seed: i32 = 0;

fn cb0(x: i64) -> i32 {
    (x ^ 0x13579L) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3L) as i32 - 11
}

fn v0(x: i32, _args:...) -> i32 {
    g_seed + x + 1000
}

fn v1(x: i32, _args:...) -> i32 {
    g_seed + x - 2000
}

fn fpfi<F>(pf: F, k: i32) -> F
where
    F: Fn(i64) -> i32,
{
    let t = (k as i64) * 97L + 1234L;
    unsafe { g_seed = pf(t) + k; }
    if (g_seed & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through<F>(pf: F, x: i32) -> i32
where
    F: Fn(i32,...) -> i32,
{
    let r1 = pf(x);
    let r2 = pf(x, 1, 2, 3);
    let r3 = pf(x, 1.25, pf, 0x1122334455667788L as i64);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);

    {
        let expected_seed = cb0((17 * 97L + 1234L) as i64) + 17;
        let base = expected_seed + 40;

        if vf == v0 {
            if vf(40)!= base + 1000 {
                println!("Expected v0(40) to return {}, but got {}", base + 1000, vf(40));
                return;
            }
            if vf(40, 1, 2)!= base + 1000 {
                println!("Expected v0(40, 1, 2) to return {}, but got {}", base + 1000, vf(40, 1, 2));
                return;
            }
            if call_through(vf, 40)!= ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                println!("Expected call_through(v0, 40) to return {}, but got {}", (base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9), call_through(vf, 40));
                return;
            }
        } else {
            if vf(40)!= base - 2000 {
                println!("Expected v1(40) to return {}, but got {}", base - 2000, vf(40));
                return;
            }
            if vf(40, 1, 2)!= base - 2000 {
                println!("Expected v1(40, 1, 2) to return {}, but got {}", base - 2000, vf(40, 1, 2));
                return;
            }
            if call_through(vf, 40)!= ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                println!("Expected call_through(v1, 40) to return {}, but got {}", (base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9), call_through(vf, 40));
                return;
            }
        }
    }

    let vf = fpfi(cb1, 8);

    {
        let expected_seed = cb1((8 * 97L + 1234L) as i64) + 8;
        let base = expected_seed + 9;

        if vf == v0 {
            if vf(9)!= base + 1000 {
                println!("Expected v0(9) to return {}, but got {}", base + 1000, vf(9));
                return;
            }
            if vf(9, 0, 0, 0, 0)!= base + 1000 {
                println!("Expected v0(9, 0, 0, 0, 0) to return {}, but got {}", base + 1000, vf(9, 0, 0, 0, 0));
                return;
            }
            if call_through(vf, 9)!= ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                println!("Expected call_through(v0, 9) to return {}, but got {}", (base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9), call_through(vf, 9));
                return;
            }
        } else {
            if vf(9)!= base - 2000 {
                println!("Expected v1(9) to return {}, but got {}", base - 2000, vf(9));
                return;
            }
            if vf(9, 0, 0, 0, 0)!= base - 2000 {
                println!("Expected v1(9, 0, 0, 0, 0) to return {}, but got {}", base - 2000, vf(9, 0, 0, 0, 0));
                return;
            }
            if call_through(vf, 9)!= ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                println!("Expected call_through(v1, 9) to return {}, but got {}", (base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9), call_through(vf, 9));
                return;
            }
        }
    }

    println!("Test passed");
}