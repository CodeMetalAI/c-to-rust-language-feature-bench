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
    let k = unsafe { &mut *(std::ptr::addr_of_mut!(v1.i) as *mut i64) };
    *k = 5;

    if v1.i != 2 {
        std::process::exit(1);
    }

    if *k != 5 {
        std::process::exit(1);
    }
}