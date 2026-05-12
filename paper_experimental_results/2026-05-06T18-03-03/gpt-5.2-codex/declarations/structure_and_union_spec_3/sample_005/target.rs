use std::process::exit;

#[repr(C)]
struct SHeader {
    i: i32,
}

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() {
    let sizeof_s = std::mem::size_of::<SHeader>();
    let offsetof_i = 0usize;
    let offsetof_a = sizeof_s;

    if offsetof_i != 0 {
        exit(1);
    }

    if offsetof_a != sizeof_s {
        exit(2);
    }

    let mut a: Vec<i32> = Vec::new();
    if a.try_reserve(4).is_err() {
        exit(3);
    }
    a.resize(4, 0);

    let mut p = S { i: 0, a };

    p.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        exit(4);
    }

    exit(0);
}