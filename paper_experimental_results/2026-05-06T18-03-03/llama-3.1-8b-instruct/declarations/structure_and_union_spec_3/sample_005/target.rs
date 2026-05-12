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

    if std::mem::offset_of::<S, Inner>() != 0 {
        panic!("offsetof(struct s, i) != 0");
    }

    if std::mem::offset_of::<S, [i32; 4]>() != std::mem::size_of::<S>() {
        panic!("offsetof(struct s, a) != sizeof(struct s)");
    }

    let p = Box::new(S {
        Inner { i: 7 },
        a: [11, 0, 0, 22],
    });

    if p.Inner.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        panic!("p->i != 7 || p->a[0] != 11 || p->a[3] != 22");
    }

    drop(p);
    std::process::exit(0);
}