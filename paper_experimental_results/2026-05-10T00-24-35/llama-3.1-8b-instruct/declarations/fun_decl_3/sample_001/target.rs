// fun_decl_3.rs

static mut G_SEED: i32 = 0;

static CB0: fn(long) -> i32 = |x| (x ^ 0x13579 as i64) as i32 + 7;

static CB1: fn(long) -> i32 = |x| (x * 3 as i64) as i32 - 11;

static V0: extern "C" fn(i32, ...) -> i32 = |x, _args: std::ptr::null_mut()| {
    unsafe { G_SEED + x + 1000 }
};

static V1: extern "C" fn(i32, ...) -> i32 = |x, _args: std::ptr::null_mut()| {
    unsafe { G_SEED + x - 2000 }
};

extern "C" {
    fn fpfi(pf: extern "C" fn(long) -> i32, k: i32) -> extern "C" fn(i32, ...) -> i32 {
        let t = k * 97 as i64 + 1234 as i64;
        unsafe { G_SEED = pf(t) + k; }
        if (G_SEED & 1 == 0) {
            V0
        } else {
            V1
        }
    }
}

extern "C" {
    fn call_through(pf: extern "C" fn(i32, ...) -> i32, x: i32) -> i32 {
        let r1 = (pf)(x);
        let r2 = (pf)(x, 1, 2, 3);
        let r3 = (pf)(x, 1.25 as i32, std::ptr::null_mut(), 0x1122334455667788 as i64);
        r1 ^ (r2 + 5) ^ (r3 + 9)
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    let vf = fpfi(CB0, 17);

    {
        let expected_seed = CB0((17 * 97 as i64 + 1234 as i64) as i64);
        let base = expected_seed + 40 as i32;

        if vf == V0 {
            if vf(40) != base + 1000 {
                return 1;
            }
            if vf(40, 1, 2) != base + 1000 {
                return 2;
            }
            if call_through(vf, 40) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 3;
            }
        } else {
            if vf(40) != base - 2000 {
                return 4;
            }
            if vf(40, 1, 2) != base - 2000 {
                return 5;
            }
            if call_through(vf, 40) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 6;
            }
        }
    }

    let vf = fpfi(CB1, 8);

    {
        let expected_seed = CB1((8 * 97 as i64 + 1234 as i64) as i64);
        let base = expected_seed + 9 as i32;

        if vf == V0 {
            if vf(9) != base + 1000 {
                return 7;
            }
            if vf(9, 0, 0, 0, 0) != base + 1000 {
                return 8;
            }
            if call_through(vf, 9) != ((base + 1000) ^ (base + 1000 + 5) ^ (base + 1000 + 9)) {
                return 9;
            }
        } else {
            if vf(9) != base - 2000 {
                return 10;
            }
            if vf(9, 0, 0, 0, 0) != base - 2000 {
                return 11;
            }
            if call_through(vf, 9) != ((base - 2000) ^ (base - 2000 + 5) ^ (base - 2000 + 9)) {
                return 12;
            }
        }
    }

    0
}