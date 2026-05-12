```rust
#[derive(Copy, Clone)]
struct Range {
    hi: f64,
    lo: f64,
}

static mut G_STORE: i32 = 7;

fn takes_int(x: i32) -> i32 {
    x + 1
}

fn takes_ptr_to_int(p: &mut i32) -> i32 {
    *p += 3;
    *p
}

fn takes_range(r: Range) -> Range {
    Range {
        hi: r.hi + 1.0,
        lo: r.lo - 1.0,
    }
}

fn takes_range_ptr(p: &Range) -> f64 {
    p.hi + p.lo
}

fn f_plain() -> i32 {
    unsafe { G_STORE }
}

fn main() -> i32 {
    let distance: i32 = 40;

    if takes_int(distance) != 41 {
        return 1;
    }

    {
        let mut t = distance;
        if t != 40 {
            return 2;
        }
        if takes_ptr_to_int(&mut t) != 43 {
            return 3;
        }
    }

    let metricp = f_plain;

    if metricp() != 7 {
        return 4;
    }

    // Rust does not allow extra arguments to a function with no parameters,
    // so we just call it without arguments.
    // The original C code allowed extra arguments (they were ignored).
    // In Rust, we can't pass extra arguments, so we just call it normally.
    if metricp() != 7 {
        return 5;
    }

    let x = Range { hi: 1.5, lo: -1.5 };

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            return 6;
        }
    }

    let z = Range { hi: 10.0, lo: -9.0 };
    let zp = &z;

    if takes_range_ptr(zp) != 1.0 {
        return 7;
    }

    {
        let copy = *zp;
        if copy.hi != 10.0 || copy.lo != -9.0 {