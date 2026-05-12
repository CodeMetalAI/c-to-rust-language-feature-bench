use std::mem::size_of;
use std::process::exit;

#[repr(C)]
struct SLayout {
    i: i32,
    a: [i32; 0],
}

fn main() {
    let offset_i = 0usize;
    if offset_i != 0 {
        exit(1);
    }

    let offset_a = size_of::<i32>();
    if offset_a != size_of::<SLayout>() {
        exit(2);
    }

    let mut a: Vec<i32> = Vec::new();
    if a.try_reserve_exact(4).is_err() {
        exit(3);
    }
    a.resize(4, 0);

    let mut i = 0i32;

    i = 7;
    a[0] = 11;
    a[3] = 22;

    if i != 7 || a[0] != 11 || a[3] != 22 {
        exit(4);
    }

    exit(0);
}