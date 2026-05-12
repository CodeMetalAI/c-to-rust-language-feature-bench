// Define our own types
#[derive(Clone, Copy)]
struct Miles(i32);

#[derive(Clone, Copy)]
struct Klicksp();

#[derive(Debug, PartialEq)]
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
    let mut out = Range { hi: r.hi + 1.0, lo: r.lo - 1.0 };
    out
}

fn takes_range_ptr(p: &Range) -> f64 {
    p.hi + p.lo
}

fn f_plain() -> i32 {
    G_STORE
}

fn main() {
    let distance: Miles = Miles(40);

    if takes_int(distance.0) != 41 {
        println!("Error 1");
        std::process::exit(1);
    }

    {
        let mut t: i32 = distance.0;
        if t != 40 {
            println!("Error 2");
            std::process::exit(2);
        }
        if takes_ptr_to_int(&mut t) != 43 {
            println!("Error 3");
            std::process::exit(3);
        }
    }

    let metricp: fn() -> i32 = f_plain;

    if (metricp)() != 7 {
        println!("Error 4");
        std::process::exit(4);
    }

    if (metricp)(1, 2, 3) != 7 {
        println!("Error 5");
        std::process::exit(5);
    }

    let mut x = Range {
        hi: 1.5,
        lo: -1.5,
    };

    {
        let y = takes_range(x);
        if (y.hi + y.lo) != (x.hi + x.lo) {
            println!("Error 6");
            std::process::exit(6);
        }
    }

    let mut z = Range {
        hi: 10.0,
        lo: -9.0,
    };
    let zp = &z;

    if takes_range_ptr(zp) != 1.0 {
        println!("Error 7");
        std::process::exit(7);
    }

    {
        let copy = *zp;
        if copy.hi != 10.0 || copy.lo != -9.0 {
            println!("Error 8");
            std::process::exit(8);
        }
    }

    println!("Success");
}