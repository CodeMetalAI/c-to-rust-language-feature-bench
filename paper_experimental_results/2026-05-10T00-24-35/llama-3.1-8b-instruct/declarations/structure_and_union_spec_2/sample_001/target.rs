fn main() {
    struct S {
        n: i32,
        d: [f64; 8],
    }

    struct SS {
        n: i32,
    }

    if std::mem::size_of::<S>() < std::mem::size_of::<SS>() {
        return;
    }

    if std::mem::offset_of::<S, [f64; 8]>()!= std::mem::size_of::<S>() {
        return;
    }

    let s1 = Box::new(S {
        n: 0,
        d: [0.0; 8],
    });

    let s2 = Box::new(S {
        n: 0,
        d: [0.0; 5],
    });

    if s1.is_none() || s2.is_none() {
        return;
    }

    let s1 = s1.unwrap();
    let s2 = s2.unwrap();

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0]!= 42.0 || s2.d[0]!= 24.0 {
        return;
    }

    drop(s1);
    drop(s2);
}