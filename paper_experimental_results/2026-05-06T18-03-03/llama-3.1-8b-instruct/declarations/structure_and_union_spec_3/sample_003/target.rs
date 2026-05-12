#[allow(unused)]
fn main() {
    let mut p = Box::new(struct_s {
        i: 0,
        a: [0; 4],
    });

    if std::mem::size_of::<struct_s>() != 4 {
        return;
    }

    if std::mem::offset_of::<struct_s, _>(struct_s::i) != 0 {
        return;
    }

    if std::mem::offset_of::<struct_s, _>(struct_s::a) != std::mem::size_of::<struct_s>() {
        return;
    }

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        return;
    }

    drop(p);
}