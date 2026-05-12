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

    if std::mem::offset_of::<S, Inner>()::i32!= 0 {
        return;
    }

    if std::mem::offset_of::<S, [i32; 4]>()::i32!= std::mem::size_of::<S>()::i32 {
        return;
    }

    let p = Box::new(S {
        Inner { i: 7 },
        a: [11, 0, 0, 22],
    });

    if p.Inner.i!= 7 || p.a[0]!= 11 || p.a[3]!= 22 {
        return;
    }

    drop(p);
    std::process::exit(0);
}