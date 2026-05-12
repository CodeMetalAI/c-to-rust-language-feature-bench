// type_def_1.rs
type MILES = i32;
type KLICKSP = fn() -> i32;

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
    let mut out = Range {
        hi: r.hi + 1.0,
        lo: r.lo - 1.0,
    };
    out
}

fn takes_range_ptr(p: &Range) -> f64 {
    p.hi + p.lo
}

fn f_plain() -> i32 {
    G_STORE
}

fn main() {
    let distance: MILES = 40;

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

    let metricp = f_plain as fn() -> i32;

    if metricp() != 7 {
        return 4;
    }

    if metricp(1, 2, 3) != 7 {
        return 5;
    }

    let mut x = Range {
        hi: 1.5,
        lo: -1.5,
    };

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            return 6;
        }
    }

    let mut z = Range {
        hi: 10.0,
        lo: -9.0,
    };
    let zp = &z;

    if takes_range_ptr(zp) != 1.0 {
        return 7;
    }

    {
        let mut copy = *zp;
        if copy.hi != 10.0 || copy.lo != -9.0 {
            return 8;
        }
    }

    return 0;
}