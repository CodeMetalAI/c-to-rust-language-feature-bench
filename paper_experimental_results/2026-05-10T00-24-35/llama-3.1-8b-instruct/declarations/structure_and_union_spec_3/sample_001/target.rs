// structure_and_union_spec_3

struct S {
    __anon_1: i32,
    a: [i32; 4],
}

fn main() {
    let p = Box::new(S {
        __anon_1: 7,
        a: [11, 0, 0, 22],
    });

    if p.__anon_1 != 7 {
        return 1;
    }

    if p.a[0] != 11 || p.a[3] != 22 {
        return 4;
    }

    Box::from_raw(p);
    std::process::exit(0);
}