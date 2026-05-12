// type_def_2.rs
struct S1 {
    x: i32,
}

struct S2 {
    x: i32,
}

fn takes_s1(v: S1) -> i32 {
    v.x + 2
}

fn takes_s1_ptr(p: &mut S1) -> i32 {
    p.x += 3;
    p.x
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_s2(v: S2) -> i32 {
    v.x + 5
}

fn main() {
    let mut a = S1 { x: 10 };
    let mut b = S1 { x: 20 };
    let mut p = &mut a;

    if takes_s1(a)!= 12 {
        panic!("Expected 12, got {}", takes_s1(a));
    }

    if takes_s1(b)!= 22 {
        panic!("Expected 22, got {}", takes_s1(b));
    }

    if takes_s1_ptr(p)!= 13 {
        panic!("Expected 13, got {}", takes_s1_ptr(p));
    }

    if a.x!= 13 {
        panic!("Expected 13, got {}", a.x);
    }

    if takes_int(a.x)!= 17 {
        panic!("Expected 17, got {}", takes_int(a.x));
    }

    let q = std::mem::size_of::<S1>() + std::mem::size_of::<S2>();
    if q == 0 {
        panic!("Expected non-zero size, got {}", q);
    }

    let mut c = S2 { x: 30 };
    let r = &mut c;
    if r.x!= 30 {
        panic!("Expected 30, got {}", r.x);
    }

    if takes_s2(c)!= 35 {
        panic!("Expected 35, got {}", takes_s2(c));
    }

    assert!(true);
}