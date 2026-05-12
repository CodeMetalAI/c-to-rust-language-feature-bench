fn cb0(x: i64) -> i32 {
    (x ^ 0x13579i64) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3i64) as i32 - 11
}

type VariadicFn = Box<dyn Fn(Vec<i32>) -> i32>;

fn fpfi(pf: fn(i64) -> i32, k: i32) -> (VariadicFn, bool) {
    let t = k as i64 * 97i64 + 1234i64;
    let seed = pf(t) + k;
    let is_v0 = (seed & 1) == 0;
    let f = move |args: Vec<i32>| {
        let x = args[0];
        if is_v0 {
            seed + x + 1000
        } else {
            seed + x - 2000
        }
    };
    (Box::new(f), is_v0)
}

fn call_through(pf: &VariadicFn, x: i32) -> i32 {
    let r1 = pf(vec![x]);
    let r2 = pf(vec![x, 1, 2, 3]);
    let r3 = pf(vec![x, 1, 2, 3]); // extras ignored, same as r2
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() -> i32 {
    let (vf, is_v0) = fpfi(cb0, 17);
    {
        let expected_seed = cb0(17i64 * 97 + 1234) + 17;
        let base = expected_seed + 40;
        if is_v0 {
            if vf(vec![40]) != base + 1000 {
                return 1;
            }
            if vf(vec![40, 1, 2]) != base + 1000 {
                return 2;
            }
            if call_through(&vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 3;
            }
        } else {
            if vf(vec![40]) != base - 2000 {
                return 4;
            }
            if vf(vec![40, 1, 2]) != base - 2000 {
                return 5;
            }
            if call_through(&vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 6;
            }
        }
    }

    let (vf, is_v0) = fpfi(cb1, 8);
    {
        let expected_seed = cb1(8i64 * 97 + 1234) + 8;
        let base = expected_seed + 9;
        if is_v0 {
            if vf(vec![9]) != base + 1000 {
                return 7;
            }
            if vf(vec![9, 0, 0, 0, 0]) != base + 1000 {
                return 8;
            }
            if call_through(&vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 9;
            }
        } else {
            if vf(vec![9]) != base - 2000 {
                return 10;
            }
            if vf(vec![9, 0, 0, 0, 0]) != base - 2000 {
                return 11;
            }
            if call_through(&vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 12;
            }
        }
    }

    0
}