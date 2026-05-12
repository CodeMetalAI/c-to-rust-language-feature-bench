struct S {
    i: i32,
    a: [i32; 4],
}

fn main() {
    let p = Box::new(S { i: 7, a: [11, 0, 0, 22] });

    if std::mem::offset_of::<S, i32>() != 0 {
        return 1;
    }

    if std::mem::offset_of::<S, [i32; 4]>() != std::mem::size_of::<S>() {
        return 2;
    }

    let size = std::mem::size_of::<S>() + 4 * std::mem::size_of::<i32>();
    let p = Box::new(S { i: 7, a: [11, 0, 0, 22] });
    let p = Box::new(std::mem::ManuallyDrop::new(p));

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        return 4;
    }

    drop(p);
    return 0;
}