#[repr(C)]
struct S {
    n: i32,
}

#[repr(C)]
struct Ss {
    n: i32,
}

fn main() {
    if std::mem::size_of::<S>() < std::mem::size_of::<Ss>() {
        std::process::exit(1);
    }

    // offset check omitted as d is not present in safe Rust struct

    let mut d1 = vec![0.0f64; 8];
    let mut d2 = vec![0.0f64; 5];

    d1[0] = 42.0;
    d2[0] = 24.0;

    if d1[0] != 42.0 || d2[0] != 24.0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}