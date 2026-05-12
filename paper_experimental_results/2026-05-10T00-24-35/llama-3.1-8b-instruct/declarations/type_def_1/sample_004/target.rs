fn main() {
    let mut distance: i32 = 40;
    if takes_int(distance) != 41 {
        println!("Test failed: takes_int");
        return;
    }

    let mut t = distance;
    if t != 40 {
        println!("Test failed: takes_int");
        return;
    }
    if takes_ptr_to_int(&mut t) != 43 {
        println!("Test failed: takes_ptr_to_int");
        return;
    }

    let metricp = f_plain;
    if metricp() != 7 {
        println!("Test failed: f_plain");
        return;
    }
    if metricp(3, 4, 5) != 7 {
        println!("Test failed: f_plain");
        return;
    }

    let mut x = Range { hi: 1.5, lo: -1.5 };
    let y = takes_range(x);
    if (y.hi + y.lo) != (x.hi + x.lo) {
        println!("Test failed: takes_range");
        return;
    }

    let mut z = Range { hi: 10.0, lo: -9.0 };
    let zp = &z;
    if takes_range_ptr(&*zp) != 1.0 {
        println!("Test failed: takes_range_ptr");
        return;
    }

    let copy = z;
    if copy.hi != 10.0 || copy.lo != -9.0 {
        println!("Test failed: Range");
        return;
    }

    assert_eq!(takes_int(distance), 41);
    assert_eq!(takes_ptr_to_int(&mut t), 43);
    assert_eq!(metricp(), 7);
    assert_eq!(metricp(3, 4, 5), 7);
    assert_eq!(takes_range(x).hi + takes_range(x).lo, x.hi + x.lo);
    assert_eq!(takes_range_ptr(&*zp), 1.0);
    assert_eq!(copy.hi, 10.0);
    assert_eq!(copy.lo, -9.0);
}

struct Range {
    hi: f64,
    lo: f64,
}

fn f_plain() -> i32 {
    7
}

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