struct S {
    struct Inner {
        i: i32,
    },
    a: [i32; 4],
}

fn main() {
    let p = Box::new(S {
        Inner { i: 7 },
        a: [0; 4],
    });

    if std::mem::offset_of::<S, _>(p).Inner.i!= 0 {
        return 1;
    }

    if std::mem::offset_of::<S, _>(p).a!= std::mem::size_of::<S>() {
        return 2;
    }

    let size = std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>();
    let p = Box::new(S {
        Inner { i: 7 },
        a: [11, 0, 0, 22],
    });

    if p.Inner.i!= 7 || p.a[0]!= 11 || p.a[3]!= 22 {
        return 4;
    }

    drop(p);
    return 0;
}