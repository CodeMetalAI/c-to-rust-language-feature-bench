#[derive(Copy, Clone)]
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
        return std::process::exit(1);
    }

    {
        let mut t = distance;
        if t != 40 {
            return std::process::exit(2);
        }
        if takes_ptr_to_int(&mut t) != 43 {
            return std::process::exit(3);
default
        }
    }

    let metricp: fn() -> i32 = f_plain;

    if metricp() != 7 {
        return std::process::exit(4);
    }

    // Rust functions cannot be called with extra arguments; 
    // the C code allowed it, but it's undefined behavior in C.
    // In Rust, we cannot call `metricp` with arguments.
    // The C test `(*metricp)(1,2,3)` would compile but is incorrect.
    // We'll emulate the same check: call the function without arguments.
    if metricp() != 7 {
        return std::process::exit(5);
    }

    let x = Range {
        hi: 1.5,
        lo: -1.5,
    };

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            return std::process::exit(6);
        }
    }

    let z = Range {
        hi: 10.0,
        lo: -9.0,
    };
    let zp = &z;

    if takes_range_ptr(zp) != 1.0 {
        return std::process::exit(7);
    }

    {
        let copy = *zp;
        if copy.hi != 10.0 || copy.lo != -9.0 {
            return std::process::exit(8);
        }
    }

    std::process::exit(0);
}