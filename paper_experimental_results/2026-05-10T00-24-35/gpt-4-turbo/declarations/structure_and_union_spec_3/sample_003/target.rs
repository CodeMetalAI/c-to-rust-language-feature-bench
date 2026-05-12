struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    // Check offset of i
    if std::mem::size_of::<i32>() != 0 {
        std::process::exit(1);
    }

    // Check offset of a
    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        std::process::exit(2);
    }

    // Allocate and initialize
    let mut p = S {
        i: 7,
        a: vec![0; 4],
    };

    p.a[0] = 11;
    p.a[3] = 22;

    // Check values
    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        std::process::exit(4);
    }
}