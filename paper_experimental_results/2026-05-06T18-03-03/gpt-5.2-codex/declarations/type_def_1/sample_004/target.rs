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

fn call_with_extra_args(f: fn() -> i32, _a: i32, _b: i32, _c: i32) -> i32 {
    f()
}

fn main() {
    let code = {
        let mut distance: i32 = 40;

        if takes_int(distance) != 41 {
            1
        } else {
            let mut t = distance;
            if t != 40 {
                2
            } else if takes_ptr_to_int(&mut t) != 43 {
                3
            } else {
                let metricp: fn() -> i32 = f_plain;

                if metricp() != 7 {
                    4
                } else if call_with_extra_args(metricp, 1, 2, 3) != 7 {
                    5
                } else {
                    let mut x = Range { hi: 1.5, lo: -1.5 };
                    let y = takes_range(x);
                    if (y.hi + y.lo) != (x.hi + x.lo) {
                        6
                    } else {
                        let z = Range { hi: 10.0, lo: -9.0 };
                        let zp: &Range = &z;

                        if takes_range_ptr(zp) != 1.0 {
                            7
                        } else {
                            let copy = *zp;
                            if copy.hi != 10.0 || copy.lo != -9.0 {
                                8
                            } else {
                                0
                            }
                        }
                    }
                }
            }
        }
    };

    exit(code);
}