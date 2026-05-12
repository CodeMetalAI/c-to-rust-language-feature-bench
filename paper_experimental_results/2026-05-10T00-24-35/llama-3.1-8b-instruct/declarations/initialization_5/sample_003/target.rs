fn main() {
    let w = [
        { let a = [1, 0, 0]; a },
        { let a = [2, 0, 0]; a },
        { let a = [0, 0, 0]; a },
        { let a = [0, 0, 0]; a },
        { let a = [0, 0, 0]; a },
        { let a = [0, 0, 0]; a },
    ];

    if w.len()!= 2 {
        panic!("sizeof(w) / sizeof(w[0])!= 2");
    }

    if w[0][0]!= 1 {
        panic!("w[0].a[0]!= 1");
    }
    if w[0][1]!= 0 {
        panic!("w[0].a[1]!= 0");
    }
    if w[0][2]!= 0 {
        panic!("w[0].a[2]!= 0");
    }
    if w[0][3]!= 0 {
        panic!("w[0].b!= 0");
    }

    if w[1][0]!= 2 {
        panic!("w[1].a[0]!= 2");
    }
    if w[1][1]!= 0 {
        panic!("w[1].a[1]!= 0");
    }
    if w[1][2]!= 0 {
        panic!("w[1].a[2]!= 0");
    }
    if w[1][3]!= 0 {
        panic!("w[1].b!= 0");
    }

    println!("0");
}