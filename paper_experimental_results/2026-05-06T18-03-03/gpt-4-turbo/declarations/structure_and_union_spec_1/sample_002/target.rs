fn main() {
    struct V {
        i: i32,
        j: i32,
        m: i32,
    }

    let mut v1 = V {
        i: 0,
        j: 0,
        m: 0,
    };

    v1.i = 2;
    // Unsafe reinterpretation of (i, j) as (k, l)
    let v1_k = unsafe { &mut *(std::ptr::addr_of_mut!(v1.i) as *mut i64) };
    *v1_k = 5;

    if v1.i != 2 {
        std::process::exit(1);
    }

    let v1_k = unsafe { *(std::ptr::addr_of!(v1.i) as *const i64) };
    if v1_k != 5 {
        std::process::exit(1);
    }
}