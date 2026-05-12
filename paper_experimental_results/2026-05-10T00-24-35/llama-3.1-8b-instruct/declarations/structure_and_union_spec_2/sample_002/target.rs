fn main() {
    struct S {
        n: i32,
        d: [f64; 8],
    };

    struct SS {
        n: i32,
    };

    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        return;
    }

    if std::mem::offset_of::<S, [f64; 8]>() != std::mem::size_of::<S>() {
        return;
    }

    let s1 = Box::new(S { n: 0, d: [0.0; 8] });
    let s2 = Box::new(S { n: 0, d: [0.0; 5] });

    if s1.as_ref().map_or(false, |s| s.d[0] != 42.0) ||
       s2.as_ref().map_or(false, |s| s.d[0] != 24.0)
    {
        return;
    }

    drop(s1);
    drop(s2);
}