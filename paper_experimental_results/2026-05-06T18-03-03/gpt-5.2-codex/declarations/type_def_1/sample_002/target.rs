#[derive(Copy, Clone)]
struct Range {
    hi: f64,
    lo: f64,
}

type Miles = i32;
type Klicksp = fn() -> i32;

const G_STORE: i32 = 7;

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

fn run() -> i32 {
    let mut distance: Miles = 0;
    let mut metricp: Klicksp;
    let mut x = Range { hi: 0.0, lo: 0.0 };
    let mut z = Range { hi: 0.0, lo: 0.0 };

    distance = 40;

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

    metricp = f_plain;

    if (metricp)() != 7 {
        return 4;
    }

    let _ = (1, 2, 3);
    if (metricp)() != 7 {
        return 5;
    }

    x.hi = 1.5;
    x.lo = -1.5;

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            return 6;
        }
    }

    z.hi = 10.0;
    z.lo = -9.0;
    let zp: &Range = &z;

    if takes_range_ptr(zp) != 1.0 {
        return 7;
    }

    {
        let copy = *zp;
        if copy.hi != 10.0 || copy.lo != -9.0 {
            return 8;
        }
    }

    0
}

fn main() {
    std::process::exit(run());
}