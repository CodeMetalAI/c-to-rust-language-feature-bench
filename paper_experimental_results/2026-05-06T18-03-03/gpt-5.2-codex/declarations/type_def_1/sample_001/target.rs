use std::process::exit;

const G_STORE: i32 = 7;

fn takes_int(x: i32) -> i32 {
    x + 1
}

fn takes_ptr_to_int(p: &mut i32) -> i32 {
    *p += 3;
    *p
}

#[derive(Copy, Clone)]
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

fn f_plain() -> i32 {
    G_STORE
}

struct MetricP {
    func: fn() -> i32,
}

impl MetricP {
    fn call0(&self) -> i32 {
        (self.func)()
    }
    fn call3(&self, _a: i32, _b: i32, _c: i32) -> i32 {
        (self.func)()
    }
}

fn main() {
    let mut distance: i32 = 0;
    let mut x = Range { hi: 0.0, lo: 0.0 };
    let mut z = Range { hi: 0.0, lo: 0.0 };

    distance = 40;

    if takes_int(distance) != 41 {
        exit(1);
    }

    {
        let mut t = distance;
        if t != 40 {
            exit(2);
        }
        if takes_ptr_to_int(&mut t) != 43 {
            exit(3);
        }
    }

    let metricp = MetricP { func: f_plain };

    if metricp.call0() != 7 {
        exit(4);
    }

    if metricp.call3(1, 2, 3) != 7 {
        exit(5);
    }

    x.hi = 1.5;
    x.lo = -1.5;

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            exit(6);
        }
    }

    z.hi = 10.0;
    z.lo = -9.0;
    let zp: &Range = &z;

    if takes_range_ptr(zp) != 1.0 {
        exit(7);
    }

    {
        let copy = *zp;
        if copy.hi != 10.0 || copy.lo != -9.0 {
            exit(8);
        }
    }

    exit(0);
}