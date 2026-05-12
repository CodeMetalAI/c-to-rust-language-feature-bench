use std::arch::asm;

static mut G_SEED: i32 = 0;

unsafe fn cb0(x: i64) -> i32 {
    (x ^ 0x13579) as i32 + 7
}

unsafe fn cb1(x: i64) -> i32 {
    (x.wrapping_mul(3)) as i32 - 11
}

// Variadic functions are not supported in safe Rust, we need to use inline assembly
// to simulate the same behavior as the C code
unsafe fn v0(x: i32, args: ...) -> i32 {
    G_SEED + x + 1000
}

unsafe fn v1(x: i32, args: ...) -> i32 {
    G_SEED + x - 2000
}

type PfLong = unsafe fn(i64) -> i32;
type Vf = unsafe fn(i32, ...) -> i32;

unsafe fn fpfi(pf: PfLong, k: i32) -> Vf {
    let t = (k as i64).wrapping_mul(97).wrapping_add(1234);
    G_SEED = pf(t) + k;
    if (G_SEED & 1) == 0 {
        v0
    } else {
        v1
    }
}

unsafe fn call_through(pf: Vf, x: i32) -> i32 {
    let r1 = pf(x);
    let r2 = pf(x, 1, 2, 3);
    let r3 = pf(x, 1.25, pf as *const (), 0x1122334455667788u64);
    r1 ^ (r2 + 5) ^ (r3 + 9)
}

fn main() -> i32 {
    unsafe {
        let mut vf: Vf;

        vf = fpfi(cb0, 17);
        {
            let expected_seed = cb0((17i64).wrapping_mul(97).wrapping_add(1234)) + 17;
            let base = expected_seed + 40;

            if vf as *const () == v0 as *const () {
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

        vf = fpfi(cb1, 8);
        {
            let expected_seed = cb1((8i64).wrapping_mul(97).wrapping_add(1234)) + 8;
            let base = expected_seed + 9;

            if vf as *const () == v0 as *const () {
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
}