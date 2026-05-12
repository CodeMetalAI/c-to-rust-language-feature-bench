type Miles = i32;

#[derive(Clone, Copy)]
struct Range {
    hi: f64,
    lo: f64,
}

trait Call {
    fn call0(&self) -> i32;
    fn call3(&self, _a: i32, _b: i32, _c: i32) -> i32;
}

struct FPlain;

impl Call for FPlain {
    fn call0(&self) -> i32 {
        7
    }
    fn call3(&self, _a: i32, _b: i32, _c: i32) -> i32 {
        7
    }
}

type KlicksP = Box<dyn Call>;

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

fn main() {
    let mut distance: Miles = 0;
    let mut metricp: Option<KlicksP> = None;
    let mut x = Range { hi: 0.0, lo: 0.0 };
    let mut z = Range { hi: 0.0, lo: 0.0 };
    let mut zp: Option<&Range> = None;

    distance = 40;

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

    metricp = Some(Box::new(FPlain));

    if metricp.as_ref().unwrap().call0() != 7 {
        std::process::exit(4);
    }

    if metricp.as_ref().unwrap().call3(1, 2, 3) != 7 {
        std::process::exit(5);
    }

    x.hi = 1.5;
    x.lo = -1.5;

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            std::process::exit(6);
        }
    }

    z.hi = 10.0;
    z.lo = -9.0;
    zp = Some(&z);

    if takes_range_ptr(zp.unwrap()) != 1.0 {
        std::process::exit(7);
    }

    {
        let copy = *zp.unwrap();
        if copy.hi != 10.0 || copy.lo != -9.0 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}