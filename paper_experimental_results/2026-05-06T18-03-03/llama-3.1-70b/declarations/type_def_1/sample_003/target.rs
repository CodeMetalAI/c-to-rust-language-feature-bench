fn main() {
    type MILES = i32;
    type KLICKSP = fn() -> i32;

    #[derive(Debug, Copy, Clone)]
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

    let mut distance: MILES = 40;
    let mut metricp: KLICKSP = f_plain;

    if takes_int(distance as i32)!= 41 {
        std::process::exit(1);
    }

    {
        let mut t = distance as i32;
        if t!= 40 {
            std::process::exit(2);
        }
        if takes_ptr_to_int(&mut t)!= 43 {
            std::process::exit(3);
        }
    }

    if metricp()!= 7 {
        std::process::exit(4);
    }

    if metricp()!= 7 {
        std::process::exit(5);
    }

    let x = Range { hi: 1.5, lo: -1.5 };

    {
        let y = takes_range(x);
        if (y.hi + y.lo)!= (x.hi + x.lo) {
            std::process::exit(6);
        }
    }

    let mut z = Range { hi: 10.0, lo: -9.0 };
    let zp = &mut z;

    if takes_range_ptr(zp)!= 1.0 {
        std::process::exit(7);
    }

    {
        let copy = *zp;
        if copy.hi!= 10.0 || copy.lo!= -9.0 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}