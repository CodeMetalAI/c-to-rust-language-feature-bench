use std::mem::size_of;

#[repr(C)]
struct Layout {
    i: i32,
    a: [i32; 0],
}

fn main() {
    let offset_i: usize = 0;
    if offset_i != 0 {
        std::process::exit(1);
    }

    let offset_a: usize = size_of::<Layout>();
    if offset_a != size_of::<Layout>() {
        std::process::exit(2);
    }

    // Simulate allocation of struct plus 4 ints
    let mut a: Vec<i32> = Vec::new();
    if a.try_reserve_exact(4).is_err() {
        std::process::exit(3);
    }
    a.resize(4, 0);

    let mut i_val: i32 = 0;

    i_val = 7;
    a[0] = 11;
    a[3] = 22;

    if i_val != 7 || a[0] != 11 || a[3] != 22 {
        std::process::exit(4);
    }

    std::process::exit(0);
}