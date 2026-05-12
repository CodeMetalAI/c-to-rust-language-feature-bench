#[repr(C)]
struct SCheck {
    n: i32,
    d: [f64; 0],
}

struct Ss {
    n: i32,
}

struct S {
    n: i32,
    d: Vec<f64>,
}

fn main() {
    if std::mem::size_of::<SCheck>() < std::mem::size_of::<Ss>() {
        std::process::exit(1);
    }

    if std::mem::offset_of!(SCheck, d) != std::mem::size_of::<SCheck>() {
        std::process::exit(1);
    }

    let mut s1 = S {
        n: 0,
        d: vec![0.0; 8],
    };
    let mut s2 = S {
        n: 0,
        d: vec![0.0; 5],
    };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}