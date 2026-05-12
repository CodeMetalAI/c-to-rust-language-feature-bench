fn main() {
    type MILES = i32;
    type KLICKSP = fn(i32, i32, i32) -> i32;

    #[derive(Copy, Clone)]
    struct range {
        hi: f64,
        lo: f64,
    }

    static mut g_store: i32 = 7;

    fn takes_int(x: i32) -> i32 { x + 1 }

    fn takes_ptr_to_int(p: &mut i32) -> i32 {
        *p += 3;
        *p
    }

    fn takes_range(r: range) -> range {
        let mut out = range { hi: r.hi + 1.0, lo: r.lo - 1.0 };
        out
    }

    fn takes_range_ptr(p: &range) -> f64 { p.hi + p.lo }

    fn f_plain() -> i32 { unsafe { g_store } }

    let mut distance: MILES = 40;

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

    let metricp: KLICKSP = f_plain;

    if metricp(1, 2, 3) != 7 {
        std::process::exit(4);
    }

    if metricp(1,2,3) != 7 {
        std::process::exit(5);
    }

    let mut x = range { hi: 1.5, lo: -1.5 };

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            std::process::exit(6);
        }
    }

    let mut z = range { hi: 10.0, lo: -9.0 };
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

    std::process::exit(0);
}