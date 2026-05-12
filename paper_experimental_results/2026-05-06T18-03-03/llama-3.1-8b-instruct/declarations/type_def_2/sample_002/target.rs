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

fn takes_s1_ptr(p: &S1) -> i32 {
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
    let a = S1 { x: 10 };
    let b = S1 { x: 20 };
    let mut p = &a;

    assert_eq!(takes_s1(a), 12);
    assert_eq!(takes_s1(b), 22);
    assert_eq!(takes_s1_ptr(&mut p), 13);
    assert_eq!(a.x, 13);
    assert_eq!(takes_int(a.x), 17);

    {
        let q = 0;
        let size_of_s1 = std::mem::size_of::<S1>();
        let size_of_s2 = std::mem::size_of::<S2>();
        if q == 0 {
            panic!("Size of S1 or S2 is not 0");
        }
    }

    {
        let c = S2 { x: 30 };
        let r = &c;
        assert_eq!(r.x, 30);
        assert_eq!(takes_s2(c), 35);
    }

    assert_eq!(0, 0);
}