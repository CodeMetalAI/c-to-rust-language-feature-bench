type Miles = i32;
type Klicksp = fn() -> i32;

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

static mut DISTANCE: Miles = 0;
static mut METRICP: Option<Klicksp> = None;
static mut X: Range = Range { hi: 0.0, lo: 0.0 };
static mut Z: Range = Range { hi: 0.0, lo: 0.0 };

fn main() {
    unsafe {
        DISTANCE = 40;

        if takes_int(DISTANCE) != 41 {
            std::process::exit(1);
        }

        {
            let mut t = DISTANCE;
            if t != 40 {
                std::process::exit(2);
            }
            if takes_ptr_to_int(&mut t) != 43 {
                std::process::exit(3);
            }
        }

        METRICP = Some(f_plain);

        if let Some(metricp) = METRICP {
            if metricp() != 7 {
                std::process::exit(4);
            }

            if metricp() != 7 {
                std::process::exit(5);
            }
        }

        X.hi = 1.5;
        X.lo = -1.5;

        {
            let y = takes_range(X);
            if (y.hi + y.lo) != (X.hi + X.lo) {
                std::process::exit(6);
            }
        }

        Z.hi = 10.0;
        Z.lo = -9.0;
        let zp = &Z;

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

    std::process::exit(0);
}