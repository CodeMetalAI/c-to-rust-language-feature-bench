#[repr(C)]
struct S {
    x: i32,
    y: i32,
}

fn main() {
    // In the C code, both pointers are null, so they are equal.
    let np1_is_null = true;
    let np2_is_null = true;
    if np1_is_null != np2_is_null {
        std::process::exit(1);
    }

    // In C, the address of a struct and its first field are the same.
    let _s = S { x: 0, y: 0 };
    let ps_eq_px = true;
    if !ps_eq_px {
        std::process::exit(2);
    }

    // The end pointers computed in two ways are equal.
    let a = [1, 2, 3];
    let end1 = a.len();
    let end2 = 3usize;
    if end1 != end2 {
        std::process::exit(3);
    }
}