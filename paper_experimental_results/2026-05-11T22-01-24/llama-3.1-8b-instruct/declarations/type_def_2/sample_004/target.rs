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

    if takes_s1(a) != 12 {
        panic!("takes_s1(a) != 12");
    }

    if takes_s1(b) != 22 {
        panic!("takes_s1(b) != 22");
    }

    if takes_s1_ptr(p) != 13 {
        panic!("takes_s1_ptr(p) != 13");
    }

    if a.x != 13 {
        panic!("a.x != 13");
    }

    if takes_int(a.x) != 17 {
        panic!("takes_int(a.x) != 17");
    }

    let q = std::mem::size_of::<S1>() + std::mem::size_of::<S2>();
    if q == 0 {
        panic!("q == 0");
    }

    let mut c = S2 { x: 30 };
    let mut r = &mut c;
    if r.x != 30 {
        panic!("r.x != 30");
    }

    if takes_s2(c) != 35 {
        panic!("takes_s2(c) != 35");
    }
}