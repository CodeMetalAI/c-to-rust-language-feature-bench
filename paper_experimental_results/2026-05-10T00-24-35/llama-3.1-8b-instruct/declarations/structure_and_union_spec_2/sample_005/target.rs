fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64; 8],
    }

    #[repr(C)]
    struct SS {
        n: i32,
    }

    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        panic!("sizeof(S) < sizeof(SS)");
    }

    let offset_of_d = std::mem::offset_of::<S>(&S { n: 0, d: [0.0; 8] }).unwrap();
    let size_of_s = std::mem::size_of::<S>();
    if offset_of_d != size_of_s {
        panic!("offsetof(S, d) != sizeof(S)");
    }

    let s1 = Box::new(S {
        n: 0,
        d: [42.0; 8],
    });
    let s2 = Box::new(S {
        n: 0,
        d: [24.0; 5],
    });

    if s1.as_ref().d[0] != 42.0 {
        panic!("s1.d[0] != 42.0");
    }
    if s2.as_ref().d[0] != 24.0 {
        panic!("s2.d[0] != 24.0");
    }

    drop(s1);
    drop(s2);
}