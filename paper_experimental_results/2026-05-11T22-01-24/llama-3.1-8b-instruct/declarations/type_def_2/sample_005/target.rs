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

    assert_eq!(takes_s1(a), 12);
    assert_eq!(takes_s1(b), 22);
    assert_eq!(takes_s1_ptr(p), 13);
    assert_eq!(a.x, 13);
    assert_eq!(takes_int(a.x), 17);

    {
        let q = 0;
        let size_of_s1 = std::mem::size_of::<S1>();
        let size_of_s2 = std::mem::size_of::<S2>();
        if q == 0 {
            panic!("size_of_s1 and size_of_s2 should not be zero");
        }
    }

    {
        let mut c = S2 { x: 30 };
        let r = &mut c;
        assert_eq!(r.x, 30);
        assert_eq!(takes_s2(c), 35);
    }

    assert_eq!(0, 0);
}