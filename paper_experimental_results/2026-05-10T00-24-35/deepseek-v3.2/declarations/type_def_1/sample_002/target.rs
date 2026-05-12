struct Range {
    hi: f64,
    lo: f64,
}

static G_STORE: i32 = 7;

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
    G_STORE
}

fn main() {
    let distance: i32 = 40;

    if takes_int(distance) != 41 {
        return;
    }

    {
        let mut t = distance;
        if t != 40 {
            return;
        }
        if takes_ptr_to_int(&mut t) != 43 {
            return;
        }
    }

    let metricp: fn() -> i32 = f_plain;

    if metricp() != 7 {
        return;
    }

    // In Rust, function pointers have fixed arity, so we cannot call with extra arguments.
    // The original C code allows calling a function pointer with extra arguments (which are ignored).
    // Since Rust doesn't allow this, we just call it normally.
    if metricp() != 7 {
        return;
    }

    let x = Range { hi: 1.5, lo: -1.5 };

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            return;
        }
    }

    let z = Range { hi: 10.0, lo: -9.0 };
    let zp = &z;

    if takes_range_ptr(zp) != 1.0 {
        return;
    }

    {
        let copy = *zp;
        if copy.hi != 10.0 || copy.lo != -9.0 {
            return;
        }
    }
}