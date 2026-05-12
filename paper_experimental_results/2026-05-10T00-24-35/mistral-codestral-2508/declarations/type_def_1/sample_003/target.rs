fn takes_int(x: i32) -> i32 {
    x + 1
}

fn takes_ptr_to_int(p: &mut i32) -> i32 {
    *p += 3;
    *p
}

#[derive(Clone, Copy)]
struct Range {
    hi: f64,
    lo: f64,
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

static G_STORE: i32 = 7;

fn f_plain() -> i32 {
    G_STORE
}

fn main() {
    let mut distance: i32 = 40;

    if takes_int(distance) != 41 {
        std::process::exit(1);
    }

    {
        let mut t = distance;
        if t != 40 {
            std::process::exit(2);
        }
        if takes_ptr_to_int(&mut t) != 43 {
            std::process::exit(3);
        }
    }

    let metricp: fn() -> i32 = f_plain;

    if metricp() != 7 {
        std::process::exit(4);
    }

    if metricp() != 7 {
        std::process::exit(5);
    }

    let x = Range { hi: 1.5, lo: -1.5 };

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            std::process::exit(6);
        }
    }

    let mut z = Range { hi: 10.0, lo: -9.0 };
    let zp = &mut z;

    if takes_range_ptr(zp) != 1.0 {
        std::process::exit(7);
    }

    {
        let copy = *zp;
        if copy.hi != 10.0 || copy.lo != -9.0 {
            std::process::exit(8);
        }
    }
}