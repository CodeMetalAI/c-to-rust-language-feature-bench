fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

fn cb1(x: i64) -> i32 {
    (x * 3) as i32 - 11
}

fn v0(x: i32, _args: &[i32]) -> i32 {
    g_seed + x + 1000
}

fn v1(x: i32, _args: &[i32]) -> i32 {
    g_seed + x - 2000
}

static mut g_seed: i32 = 0;

type Callback = fn(i64) -> i32;
type VarargFunc = fn(i32, &[i32]) -> i32;

fn fpfi(pf: Callback, k: i32) -> VarargFunc {
    let t = (k as i64) * 97 + 1234;
    unsafe {
        g_seed = pf(t) + k;
    }
    if (unsafe { g_seed } & 1) == 0 {
        v0
    } else {
        v1
    }
}

fn call_through(pf: VarargFunc, x: i32) -> i32 {
    let r1 = pf(x, &[]);
    let r2 = pf(x, &[1, 2, 3]);
    let r3 = pf(x, &[1, 2, 3, 4]);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() {
    let vf = fpfi(cb0, 17);

    let expected_seed = cb0(17 as i64 * 97 + 1234);
    let base = expected_seed + 40;

    if vf == v0 {
        assert_eq!(vf(40, &[]), base + 1000);
        assert_eq!(vf(40, &[1, 2]), base + 1000);
        assert_eq!(call_through(vf, 40), (base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9));
    } else {
        assert_eq!(vf(40, &[]), base - 2000);
        assert_eq!(vf(40, &[1, 2]), base - 2000);
        assert_eq!(call_through(vf, 40), (base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9));
    }

    let vf = fpfi(cb1, 8);

    let expected_seed = cb1(8 as i64 * 97 + 1234);
    let base = expected_seed + 9;

    if vf == v0 {
        assert_eq!(vf(9, &[]), base + 1000);
        assert_eq!(vf(9, &[0, 0, 0, 0]), base + 1000);
        assert_eq!(call_through(vf, 9), (base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9));
    } else {
        assert_eq!(vf(9, &[]), base - 2000);
        assert_eq!(vf(9, &[0, 0, 0, 0]), base - 2000);
        assert_eq!(call_through(vf, 9), (base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9));
    }
}